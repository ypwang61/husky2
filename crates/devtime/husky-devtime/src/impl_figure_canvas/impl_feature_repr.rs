use husky_comptime::utils::__RegisterDowncastResult;
use husky_vm_primitive_value::PrimitiveValueData;

use super::*;

impl HuskyDevtime {
    pub(crate) fn feature_repr_figure(
        &self,
        repr: &FeatureRepr,
        opt_arrival_indicator: Option<&Arc<FeatureArrivalIndicator>>,
        is_specific: bool,
    ) -> Result<FigureCanvasData, (SampleId, __VMError)> {
        if is_specific {
            match self.runtime().visualize_feature(
                repr.clone(),
                opt_arrival_indicator,
                self.state.presentation.sample_id(),
            ) {
                Ok(data) => Ok(FigureCanvasData::new_specific(data)),
                Err(_) => Ok(FigureCanvasData::void()),
            }
        } else {
            self.feature_repr_generic_figure(repr, opt_arrival_indicator)
        }
    }

    fn feature_repr_generic_figure(
        &self,
        repr: &FeatureRepr,
        opt_arrival_indicator: Option<&Arc<FeatureArrivalIndicator>>,
    ) -> Result<FigureCanvasData, (SampleId, __VMError)> {
        // const COLUMN_HEIGHT: u32 = 5;
        let ty = repr.ty();
        let visualizer = self.runtime().visualizer(ty.intrinsic());
        match visualizer.visual_ty {
            VisualTy::Void => Ok(FigureCanvasData::void()),
            VisualTy::Bool => todo!(),
            VisualTy::B32 => todo!(),
            VisualTy::B64 => todo!(),
            VisualTy::Integer => {
                let ref this = self;
                Ok(FigureCanvasData::GenericI32 {
                    partitioned_samples: this.feature_repr_partitioned_samples(
                        repr,
                        opt_arrival_indicator,
                        |visual_data| match visual_data {
                            VisualData::Primitive {
                                value: PrimitiveValueData::I32(i),
                            } => i,
                            _ => {
                                p!(visual_data);
                                panic!()
                            }
                        },
                    )?,
                })
            }
            VisualTy::Float => Ok(FigureCanvasData::GenericF32 {
                partitioned_samples: self.feature_repr_partitioned_samples(
                    repr,
                    opt_arrival_indicator,
                    |visual_data| match visual_data {
                        VisualData::Primitive {
                            value: PrimitiveValueData::F32(f),
                        } => f,
                        _ => panic!(),
                    },
                )?,
            }),
            VisualTy::Point2d => todo!(),
            VisualTy::Shape2d | VisualTy::Region2d | VisualTy::Image2d | VisualTy::Graphics2d => {
                Ok(FigureCanvasData::GenericGraphics2d {
                    partitioned_samples: self.feature_repr_partitioned_samples(
                        repr,
                        opt_arrival_indicator,
                        |visual_data| Graphics2dCanvasData::from_visual_data(visual_data),
                    )?,
                })
            }
            VisualTy::Dataset => todo!(),
            VisualTy::Plot2d => todo!(),
            VisualTy::Any => todo!(),
            VisualTy::AnyGroup => todo!(),
            VisualTy::ThickFp => todo!(),
        }
    }

    fn feature_repr_partitioned_samples<T>(
        &self,
        repr: &FeatureRepr,
        opt_arrival_indicator: Option<&Arc<FeatureArrivalIndicator>>,
        transform_visual_data: impl Fn(VisualData) -> T,
    ) -> Result<Vec<(PartitionDefnData, Vec<(SampleId, T)>)>, (SampleId, __VMError)> {
        let session = self.runtime().session();
        let dev_division = session.dev();
        let presentation = &self.state.presentation;
        let mut sampler = PartitionedSampler::<T>::new(presentation.partitions());
        for labeled_data in dev_division.each_labeled_data() {
            let label = labeled_data.label;
            let sample_id = labeled_data.sample_id;
            let f = |e| (sample_id, e);
            if !self
                .runtime()
                .eval_opt_arrival_indicator_cached(opt_arrival_indicator, sample_id)
                .map_err(f)?
            {
                continue;
            }
            if !self
                .is_restriction_satisfied(presentation, sample_id)
                .map_err(f)?
            {
                continue;
            }
            if sampler
                .process(label, || {
                    let visual_data =
                        self.runtime()
                            .visualize_feature(repr.clone(), None, sample_id)?;
                    Ok((labeled_data.sample_id, transform_visual_data(visual_data)))
                })
                .map_err(f)?
            {
                break;
            }
        }
        Ok(sampler.finish())
    }

    fn is_restriction_satisfied(
        &self,
        presentation: &Presentation,
        sample_id: SampleId,
    ) -> __VMResult<bool> {
        match presentation.restriction() {
            Restriction::None => Ok(true),
            Restriction::Arrival {
                trace_id,
                feature_id,
                arrival_restriction_kind,
            } => self.is_arrival_restriction_satisfied(
                trace_id,
                arrival_restriction_kind,
                presentation.partitions(),
                sample_id,
            ),
        }
    }

    fn is_arrival_restriction_satisfied(
        &self,
        trace_id: TraceId,
        arrival_restriction_kind: ArrivalRestrictionKind,
        partitions: &Partitions,
        sample_id: SampleId,
    ) -> __VMResult<bool> {
        match arrival_restriction_kind {
            ArrivalRestrictionKind::Default => self.is_trace_arrived(trace_id, sample_id),
            ArrivalRestrictionKind::Return => todo!(),
            ArrivalRestrictionKind::DeprecatedStrikeEvil => {
                self.is_trace_striking_evil(trace_id, sample_id, partitions)
            }
        }
    }

    fn is_trace_arrived(&self, trace_id: TraceId, sample_id: SampleId) -> __VMResult<bool> {
        let trace = self.trace(trace_id);
        match trace.variant {
            TraceVariant::Main(_) => Ok(true),
            TraceVariant::Module { .. } => panic!(),
            TraceVariant::EntityFeature { .. } => Ok(true),
            TraceVariant::FeatureStmt(ref stmt) => self
                .runtime()
                .eval_opt_arrival_indicator_cached(stmt.opt_arrival_indicator.as_ref(), sample_id),
            TraceVariant::FeatureBranch(ref branch) => {
                self.runtime().eval_opt_arrival_indicator_cached(
                    branch.opt_arrival_indicator.as_ref(),
                    sample_id,
                )
            }
            TraceVariant::FeatureExpr(_) => todo!(),
            TraceVariant::FeatureCallArgument { .. } => todo!(),
            TraceVariant::FuncStmt { .. } => todo!(),
            TraceVariant::ProcStmt { .. } => todo!(),
            TraceVariant::ProcBranch { .. } => todo!(),
            TraceVariant::FuncBranch { .. } => todo!(),
            TraceVariant::LoopFrame { .. } => todo!(),
            TraceVariant::EagerExpr { .. } => todo!(),
            TraceVariant::EagerCallArgument { .. } => todo!(),
            TraceVariant::CallHead { .. } => todo!(),
        }
    }

    fn is_trace_striking_evil(
        &self,
        trace_id: TraceId,
        sample_id: SampleId,
        partitions: &Partitions,
    ) -> __VMResult<bool> {
        let trace = self.trace(trace_id);
        let (value, ty) = match trace.variant {
            TraceVariant::Main(ref repr) => {
                (self.runtime.eval_feature_repr(repr, sample_id)?, repr.ty())
            }
            TraceVariant::EntityFeature { ref repr, .. } => {
                (self.runtime.eval_feature_repr(repr, sample_id)?, repr.ty())
            }
            TraceVariant::FeatureStmt(ref stmt) => (
                self.runtime.eval_feature_stmt(stmt, sample_id)?,
                stmt.return_ty,
            ),
            TraceVariant::FeatureBranch(ref branch) => (
                self.runtime.eval_feature_lazy_branch(branch, sample_id)?,
                branch.block.return_ty.route,
            ),
            TraceVariant::FeatureExpr(_) => todo!(),
            TraceVariant::FeatureCallArgument { .. } => todo!(),
            TraceVariant::FuncStmt { .. } => todo!(),
            TraceVariant::ProcStmt { .. } => todo!(),
            TraceVariant::ProcBranch { .. } => todo!(),
            TraceVariant::FuncBranch { .. } => todo!(),
            TraceVariant::LoopFrame { .. } => todo!(),
            TraceVariant::EagerExpr { .. } => todo!(),
            TraceVariant::EagerCallArgument { .. } => todo!(),
            TraceVariant::Module { .. } | TraceVariant::CallHead { .. } => panic!(),
        };
        assert!(ty == self.runtime().target_output_ty().unwrap());
        let label_downcast_result = self.runtime().register_to_label_converter()(&value);
        let true_label = self.runtime.session().dev().label(sample_id);
        match label_downcast_result {
            __RegisterDowncastResult::Value(predicted_label) => Ok(predicted_label != true_label),
            __RegisterDowncastResult::None { number_of_somes } => {
                if number_of_somes != 0 {
                    todo!()
                }
                Ok(partitions.is_nondefault(true_label))
            }
            __RegisterDowncastResult::Unreturned => Ok(false),
        }
    }
}
