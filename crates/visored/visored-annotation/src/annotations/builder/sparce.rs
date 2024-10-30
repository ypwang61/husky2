use crate::{
    annotation::{space::VdSpaceAnnotation, token::VdTokenAnnotation},
    annotations::{
        VdAnnotationEntry, VdAnnotations, VdSpaceAnnotationEntry, VdTokenAnnotationEntry,
    },
};

pub(crate) fn collect_from_sparse_annotations<'a>(
    raw_text: &'a str,
    token_annotation_iter: impl Iterator<Item = ((&'a str, &'a str), VdTokenAnnotation)>,
    space_annotation_iter: impl Iterator<Item = ((&'a str, &'a str), VdSpaceAnnotation)>,
) -> VdAnnotations {
    let token_annotations = collect_from_sparse_token_annotations(raw_text, token_annotation_iter);
    let space_annotations = collect_from_sparse_space_annotations(raw_text, space_annotation_iter);

    VdAnnotations::new(token_annotations, space_annotations)
}

fn collect_from_sparse_token_annotations<'a>(
    raw_text: &'a str,
    token_annotation_iter: impl Iterator<Item = ((&'a str, &'a str), VdTokenAnnotation)>,
) -> Vec<VdTokenAnnotationEntry> {
    collect_from_sparse_annotations_aux(raw_text, token_annotation_iter)
}

fn collect_from_sparse_space_annotations<'a>(
    raw_text: &'a str,
    space_annotation_iter: impl Iterator<Item = ((&'a str, &'a str), VdSpaceAnnotation)>,
) -> Vec<VdSpaceAnnotationEntry> {
    collect_from_sparse_annotations_aux(raw_text, space_annotation_iter)
}

fn collect_from_sparse_annotations_aux<'a, A>(
    raw_text: &'a str,
    annotation_iter: impl Iterator<Item = ((&'a str, &'a str), A)>,
) -> Vec<VdAnnotationEntry<A>> {
    let mut annotations = Vec::new();
    for ((prev_s, token_s), annotation) in annotation_iter {
        let start = prev_s.len();
        let end = prev_s.len() + token_s.len();
        assert_eq!(&raw_text[..start], prev_s);
        assert_eq!(&raw_text[start..end], token_s);
        annotations.push(VdAnnotationEntry {
            start,
            end,
            annotation,
        });
    }
    annotations
}

#[cfg(test)]
mod tests {
    use crate::annotation::{
        space::LxApplyAnnotation,
        token::{LxIntegralAnnotation, LxVariableAnnotation},
    };
    use expect_test::expect;

    use super::*;

    #[test]
    fn test_collect_from_sparse_annotations_integral() {
        let input = "\\int xdx";

        let token_annotations = vec![
            (
                ("", "\\int"),
                VdTokenAnnotation::Integral(
                    LxIntegralAnnotation::SingleVariableIndefiniteIntegralOverReal,
                ),
            ),
            (
                ("\\int ", "x"),
                VdTokenAnnotation::Variable(LxVariableAnnotation::Usage),
            ),
            (("\\int x", "d"), VdTokenAnnotation::Differential),
            (
                ("\\int xd", "x"),
                VdTokenAnnotation::Variable(
                    LxVariableAnnotation::SingleVariableIntegralVariableDecl,
                ),
            ),
        ];

        let space_annotations = vec![(
            ("\\int x", "d"),
            VdSpaceAnnotation::Apply(LxApplyAnnotation::ScalarDifferentialFormMul),
        )];

        let result = collect_from_sparse_annotations(
            input,
            token_annotations.iter().copied(),
            space_annotations.iter().copied(),
        );

        expect![[r#"
            VdAnnotations {
                token_annotations: [
                    VdAnnotationEntry {
                        start: 0,
                        end: 4,
                        annotation: Integral(
                            SingleVariableIndefiniteIntegralOverReal,
                        ),
                    },
                    VdAnnotationEntry {
                        start: 5,
                        end: 6,
                        annotation: Variable(
                            Usage,
                        ),
                    },
                    VdAnnotationEntry {
                        start: 6,
                        end: 7,
                        annotation: Differential,
                    },
                    VdAnnotationEntry {
                        start: 7,
                        end: 8,
                        annotation: Variable(
                            SingleVariableIntegralVariableDecl,
                        ),
                    },
                ],
                space_annotations: [
                    VdAnnotationEntry {
                        start: 6,
                        end: 7,
                        annotation: Apply(
                            ScalarDifferentialFormMul,
                        ),
                    },
                ],
            }
        "#]]
        .assert_debug_eq(&result);
    }
}
