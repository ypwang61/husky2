use husky_core::*;
use ad_hoc_task_dependency::{*, ugly::*};
use mnist_classifier::*;

#[rustfmt::skip]
linkage_impls![
    mnist_classifier::main,
    mnist_classifier::connected_component::ConnectedComponentDistribution::__constructor,
    mnist_classifier::connected_component::EffHoles::__constructor,
    mnist_classifier::connected_component::hole_tmpl,
    mnist_classifier::connected_component::ConnectedComponent::__constructor,
    mnist_classifier::connected_component::horizontal_extend,
    mnist_classifier::connected_component::find_connected_components,
    mnist_classifier::connected_component::ConnectedComponent::raw_contours,
    mnist_classifier::connected_component::ConnectedComponent::eff_holes,
    mnist_classifier::connected_component::ConnectedComponent::max_hole_ilen,
    mnist_classifier::connected_component::ConnectedComponent::max_row_span,
    mnist_classifier::connected_component::ConnectedComponent::row_span_sum,
    mnist_classifier::connected_component::ConnectedComponent::distribution,
    mnist_classifier::connected_component::ConnectedComponent::upper_mass,
    mnist_classifier::connected_component::ConnectedComponent::lower_mass,
    <mnist_classifier::connected_component::ConnectedComponent>::top_k_row_span_sum,
    <mnist_classifier::connected_component::ConnectedComponent>::top_k_row_right_mass_sum,
    mnist_classifier::raw_contour::RawContour::__constructor,
    mnist_classifier::raw_contour::get_pixel_pair,
    mnist_classifier::raw_contour::get_pixel_to_the_left,
    mnist_classifier::raw_contour::get_pixel_to_the_right,
    mnist_classifier::raw_contour::get_inward_direction,
    mnist_classifier::raw_contour::get_angle_change,
    mnist_classifier::raw_contour::get_outward_direction,
    mnist_classifier::raw_contour::StreakCache::__constructor,
    mnist_classifier::raw_contour::get_concave_middle_point,
    mnist_classifier::raw_contour::find_raw_contours,
    mnist_classifier::raw_contour::RawContour::line_segment_sketch,
    mnist_classifier::raw_contour::RawContour::bounding_box,
    mnist_classifier::raw_contour::RawContour::relative_bounding_box,
    mnist_classifier::raw_contour::RawContour::contour_len,
    <mnist_classifier::raw_contour::RawContour>::displacement,
    mnist_classifier::geom2d::Point2d::__constructor,
    mnist_classifier::geom2d::RelativePoint2d::__constructor,
    mnist_classifier::geom2d::Vector2d::__constructor,
    mnist_classifier::geom2d::ClosedRange::__constructor,
    mnist_classifier::geom2d::BoundingBox::__constructor,
    mnist_classifier::geom2d::RelativeBoundingBox::__constructor,
    <mnist_classifier::geom2d::Point2d>::from_i_shift28,
    <mnist_classifier::geom2d::Point2d>::vector,
    <mnist_classifier::geom2d::Point2d>::to,
    <mnist_classifier::geom2d::Point2d>::norm,
    <mnist_classifier::geom2d::Point2d>::dist,
    <mnist_classifier::geom2d::Vector2d>::point,
    <mnist_classifier::geom2d::Vector2d>::to,
    <mnist_classifier::geom2d::Vector2d>::norm,
    <mnist_classifier::geom2d::Vector2d>::dot,
    <mnist_classifier::geom2d::Vector2d>::cross,
    <mnist_classifier::geom2d::Vector2d>::angle,
    <mnist_classifier::geom2d::Vector2d>::rotation_direction_to,
    <mnist_classifier::geom2d::Vector2d>::angle_to,
    <mnist_classifier::geom2d::ClosedRange>::relative_range,
    <mnist_classifier::geom2d::ClosedRange>::relative_point,
    <mnist_classifier::geom2d::BoundingBox>::relative_bounding_box,
    <mnist_classifier::geom2d::BoundingBox>::relative_point,
    <mnist_classifier::geom2d::BoundingBox>::xmin,
    <mnist_classifier::geom2d::BoundingBox>::xmax,
    <mnist_classifier::geom2d::BoundingBox>::ymin,
    <mnist_classifier::geom2d::BoundingBox>::ymax,
    <mnist_classifier::geom2d::RelativeBoundingBox>::xmin,
    <mnist_classifier::geom2d::RelativeBoundingBox>::xmax,
    <mnist_classifier::geom2d::RelativeBoundingBox>::ymin,
    <mnist_classifier::geom2d::RelativeBoundingBox>::ymax,
    mnist_classifier::line_segment_sketch::LineSegmentStroke::__constructor,
    mnist_classifier::line_segment_sketch::LineSegmentSketch::__constructor,
    mnist_classifier::line_segment_sketch::go_right,
    mnist_classifier::line_segment_sketch::go_left,
    mnist_classifier::line_segment_sketch::extend_end,
    mnist_classifier::line_segment_sketch::extend_start,
    mnist_classifier::line_segment_sketch::find_line_segments,
    <mnist_classifier::line_segment_sketch::LineSegmentStroke>::new,
    <mnist_classifier::line_segment_sketch::LineSegmentStroke>::displacement,
    mnist_classifier::line_segment_sketch::LineSegmentSketch::concave_components,
    mnist_classifier::line_segment_sketch::LineSegmentSketch::bounding_box,
    <mnist_classifier::line_segment_sketch::LineSegmentSketch>::new,
    mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent::__constructor,
    mnist_classifier::line_segment_sketch::concave_component::find_concave_components,
    mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent::norm,
    mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent::rel_norm,
    mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent::hausdorff_norm,
    mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent::angle_change,
    mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent::bounding_box,
    mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent::relative_bounding_box,
    <mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent>::line_segment,
    <mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent>::start,
    <mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent>::end,
    <mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent>::displacement,
    <mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent>::start_tangent,
    <mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent>::end_tangent,
    mnist_classifier::line_segment_sketch::convex_component::ConvexComponent::__constructor,
    mnist_classifier::line_segment_sketch::convexity::is_convex,
    mnist_classifier::line_segment_sketch::line_segment::LineSegment::__constructor,
    <mnist_classifier::line_segment_sketch::line_segment::LineSegment>::displacement,
    <mnist_classifier::line_segment_sketch::line_segment::LineSegment>::dist_to_point,
    mnist_classifier::fermi::FermiMatchResult::__constructor,
    mnist_classifier::fermi::fermi_match,
    mnist_classifier::fermi::FermiMatchResult::norm,
    mnist_classifier::fermi::FermiMatchResult::rel_norm,
    mnist_classifier::fermi::FermiMatchResult::angle_change_norm,
    mnist_classifier::digits::zero::open_one_match,
    mnist_classifier::digits::zero::almost_closed,
    mnist_classifier::digits::zero::is_zero,
    mnist_classifier::digits::one::one_fermi_match,
    mnist_classifier::digits::one::is_one,
    mnist_classifier::digits::one::upmost,
    mnist_classifier::digits::one::downmost,
    mnist_classifier::digits::one::hat,
    mnist_classifier::digits::six::six_match,
    mnist_classifier::digits::six::six_match_refined1,
    mnist_classifier::digits::six::is_six,
    mnist_classifier::digits::six::upmost,
    mnist_classifier::digits::six::bottom1,
    mnist_classifier::digits::three::three_fermi_match,
    mnist_classifier::digits::three::is_three,
    mnist_classifier::digits::three::uparc,
    mnist_classifier::digits::three::downarc,
    mnist_classifier::digits::three::back,
    mnist_classifier::digits::four::left_components,
    mnist_classifier::digits::four::left_coordinate_max,
    mnist_classifier::digits::four::components_max_downwards,
    mnist_classifier::digits::four::components_max_heights,
    mnist_classifier::digits::four::is_four,
    mnist_classifier::digits::four::displacement_downwards,
    mnist_classifier::digits::four::cc_box_heights,
    mnist_classifier::digits::five::is_five,
    mnist_classifier::digits::seven::simple_seven_match,
    mnist_classifier::digits::seven::simple_leftdown_pattern,
    mnist_classifier::digits::seven::special_seven_match,
    mnist_classifier::digits::seven::leftupcc_pattern,
    mnist_classifier::digits::seven::leftdowncc_pattern,
    mnist_classifier::digits::seven::is_seven,
    mnist_classifier::digits::eight::upper_mouth_match,
    mnist_classifier::digits::eight::is_eight,
    mnist_classifier::digits::eight::big_mouth,
    mnist_classifier::digits::nine::nine_match,
    mnist_classifier::digits::nine::nine_match_refine,
    mnist_classifier::digits::nine::is_nine,
    mnist_classifier::digits::nine::downmost,
    mnist_classifier::digits::nine::big_cc,
    mnist_classifier::digits::two::two_match,
    mnist_classifier::digits::two::left_cc_pattern,
    mnist_classifier::digits::two::right_cc_pattern,
    mnist_classifier::digits::two::down_cc_pattern,
    mnist_classifier::digits::two::is_two,
    mnist_classifier::major::connected_components,
    mnist_classifier::major::major_connected_component,
    mnist_classifier::major::ignored_connected_components_row_span_sum_sum,
    mnist_classifier::major::major_raw_contours,
    mnist_classifier::major::major_raw_contour,
    mnist_classifier::major::major_line_segment_sketch,
    mnist_classifier::major::major_concave_components,
    <mnist::BinaryImage28 as Clone>::clone,
    <Vec<mnist_classifier::connected_component::ConnectedComponent>>::push,
    <Vec<mnist_classifier::raw_contour::RawContour>>::collect_leashes,
    <Vec<Leash<mnist_classifier::raw_contour::RawContour>>>::pop_with_largest_opt_f32,
    <Vec<Option<Leash<mnist_classifier::raw_contour::RawContour>>>>::push,
    <Vec<mnist_classifier::raw_contour::RawContour>>::ilen,
    <Vec<mnist_classifier::geom2d::Point2d>>::ilen,
    <[mnist_classifier::geom2d::Point2d]>::last,
    <[mnist_classifier::geom2d::Point2d]>::last_mut,
    <Vec<mnist_classifier::geom2d::Point2d>>::push,
    <Vec<mnist_classifier::geom2d::Point2d>>::pop,
    <Vec<mnist_classifier::raw_contour::RawContour>>::push,
    <CyclicSliceLeashed<mnist_classifier::geom2d::Point2d>>::first,
    <mnist_classifier::geom2d::Point2d as Clone>::clone,
    <CyclicSliceLeashed<mnist_classifier::geom2d::Point2d>>::last,
    <Vec<mnist_classifier::line_segment_sketch::LineSegmentStroke>>::ilen,
    <[mnist_classifier::line_segment_sketch::LineSegmentStroke]>::last,
    <[mnist_classifier::line_segment_sketch::LineSegmentStroke]>::last_mut,
    <CyclicSliceLeashed<mnist_classifier::geom2d::Point2d>>::start,
    <Vec<mnist_classifier::line_segment_sketch::LineSegmentStroke>>::pop,
    <CyclicSliceLeashed<mnist_classifier::geom2d::Point2d>>::end,
    <Vec<mnist_classifier::line_segment_sketch::LineSegmentStroke>>::push,
    <[mnist_classifier::line_segment_sketch::LineSegmentStroke]>::first,
    <[mnist_classifier::line_segment_sketch::LineSegmentStroke]>::first_mut,
    <Vec<mnist_classifier::geom2d::Point2d>>::cyclic_slice_leashed,
    <Vec<mnist_classifier::line_segment_sketch::LineSegmentStroke>>::cyclic_slice_leashed,
    <Vec<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent>>::push,
    <CyclicSliceLeashed<mnist_classifier::line_segment_sketch::LineSegmentStroke>>::first,
    <CyclicSliceLeashed<mnist_classifier::line_segment_sketch::LineSegmentStroke>>::start,
    <CyclicSliceLeashed<mnist_classifier::line_segment_sketch::LineSegmentStroke>>::end,
    <CyclicSliceLeashed<mnist_classifier::line_segment_sketch::LineSegmentStroke>>::last,
    <Vec<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent>>::collect_leashes,
    <Vec<fn(/* ad hoc*/)>>::ilen,
    <Vec<Leash<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent>>>::pop_with_largest_opt_f32,
    <Vec<Option<Leash<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent>>>>::push,
    <Vec<Leash<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent>>>::ilen,
    <Vec<mnist_classifier::connected_component::ConnectedComponent>>::ilen,
    malamute::narrow_down,
    <Vec<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent>>::ilen,
];