use ratatui::layout::Rect;

/// Returns a `width`×`height` rect centered in `area`, clamped to `area`.
pub fn centered_rect(area: Rect, width: u16, height: u16) -> Rect {
    let w = width.min(area.width);
    let h = height.min(area.height);
    Rect::new(
        area.x + (area.width - w) / 2,
        area.y + (area.height - h) / 2,
        w,
        h,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn centers_inside_larger_area() {
        let area = Rect::new(10, 5, 100, 40);
        assert_eq!(centered_rect(area, 20, 10), Rect::new(50, 20, 20, 10));
    }

    #[test]
    fn clamps_to_area_when_too_big() {
        let area = Rect::new(0, 0, 30, 8);
        assert_eq!(centered_rect(area, 50, 20), area);
    }
}
