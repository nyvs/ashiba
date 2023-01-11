use tui::layout::{Rect, Layout, Direction, Constraint};

pub struct BasicPlacement;

impl BasicPlacement {
    pub fn compute_area_relative(area: Rect, pos_percentage: (u16, u16), size: (u16, u16)) -> Rect {
        assert!((0..=100).contains(&pos_percentage.0) && (0..=100).contains(&pos_percentage.1), "pos_percentage must be between 0 and 100");
        Self::relative_rect(area, pos_percentage, size)
    }

    pub fn compute_area_at_pos(area: Rect, pos: (u16, u16), size: (u16, u16)) -> Option<Rect> {
        let raw = Self::rect_at_pos(pos, size);
        //raw rect may not exceed area
        if raw.x + raw.width <= area.width && raw.y + raw.height <= area.height {
            Some(raw)
        } else {
            None
        }
    }

    fn relative_rect(area: Rect, pos_percentage: (u16, u16), size: (u16, u16)) -> Rect {
        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(((area.height as f32 * (pos_percentage.1 as f32/100f32)) as u16) - size.1/2),
                    Constraint::Length(size.1),
                    Constraint::Length(((area.height as f32 * (pos_percentage.1 as f32/100f32)) as u16) - size.1/2),
                ]
                .as_ref(),
            )
            .split(area);
    
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Length(((area.width as f32 * (pos_percentage.0 as f32/100f32)) as u16) - size.0/2),
                    Constraint::Length(size.0),
                    Constraint::Length(((area.width as f32 * (pos_percentage.0 as f32/100f32)) as u16) - size.0/2),
                ]
                .as_ref(),
            )
            .split(popup_layout[1])[1]
    }

    fn rect_at_pos(pos: (u16, u16), size: (u16, u16)) -> Rect {
        Rect {
            x: pos.0,
            y: pos.1,
            width: size.0,
            height: size.1,
        }
    }
}
