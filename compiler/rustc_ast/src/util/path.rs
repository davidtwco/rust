use rustc_span::{Symbol, sym};

use crate::{GenericBound, TraitBoundModifiers, ast};

pub fn is_maybe_sized_bound(bound: &GenericBound) -> bool {
    if let GenericBound::Trait(trait_ref) = bound
        && let TraitBoundModifiers { polarity: ast::BoundPolarity::Maybe(_), .. } =
            trait_ref.modifiers
        && is_sized_marker(&trait_ref.trait_ref.path)
    {
        true
    } else {
        false
    }
}

pub fn is_maybe_metasized_bound(bound: &GenericBound) -> bool {
    if let GenericBound::Trait(trait_ref) = bound
        && let TraitBoundModifiers { polarity: ast::BoundPolarity::Maybe(_), .. } =
            trait_ref.modifiers
        && is_metasized_marker(&trait_ref.trait_ref.path)
    {
        true
    } else {
        false
    }
}

pub fn contains_maybe_sized_bound(bounds: &[GenericBound]) -> bool {
    bounds.iter().any(is_maybe_sized_bound)
}

fn path_segment_is_exact_match(path_segments: &[ast::PathSegment], syms: &[Symbol]) -> bool {
    path_segments.iter().zip(syms).all(|(segment, &symbol)| segment.ident.name == symbol)
}

fn is_sized_marker(path: &ast::Path) -> bool {
    const CORE_UNSIZE: [Symbol; 3] = [sym::core, sym::marker, sym::Sized];
    const STD_UNSIZE: [Symbol; 3] = [sym::std, sym::marker, sym::Sized];
    if path.segments.len() == 4 && path.is_global() {
        path_segment_is_exact_match(&path.segments[1..], &CORE_UNSIZE)
            || path_segment_is_exact_match(&path.segments[1..], &STD_UNSIZE)
    } else if path.segments.len() == 3 {
        path_segment_is_exact_match(&path.segments, &CORE_UNSIZE)
            || path_segment_is_exact_match(&path.segments, &STD_UNSIZE)
    } else {
        *path == sym::Sized
    }
}

fn is_metasized_marker(path: &ast::Path) -> bool {
    const CORE_UNSIZE: [Symbol; 3] = [sym::core, sym::marker, sym::MetaSized];
    const STD_UNSIZE: [Symbol; 3] = [sym::std, sym::marker, sym::MetaSized];
    if path.segments.len() == 4 && path.is_global() {
        path_segment_is_exact_match(&path.segments[1..], &CORE_UNSIZE)
            || path_segment_is_exact_match(&path.segments[1..], &STD_UNSIZE)
    } else if path.segments.len() == 3 {
        path_segment_is_exact_match(&path.segments, &CORE_UNSIZE)
            || path_segment_is_exact_match(&path.segments, &STD_UNSIZE)
    } else {
        *path == sym::MetaSized
    }
}
