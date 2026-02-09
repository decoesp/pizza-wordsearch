use printpdf::*;
use std::fs::File;
use std::io::BufWriter;

use crate::grid::{Grid, PlacementResult};

const PAGE_WIDTH_MM: f32 = 210.0;
const PAGE_HEIGHT_MM: f32 = 297.0;
const MARGIN_MM: f32 = 20.0;
const CELL_SIZE_MM: f32 = 8.0;

pub struct PdfGenerator {
    title: String,
}

impl PdfGenerator {
    pub fn new(title: &str) -> Self {
        Self { title: title.to_string() }
    }

    pub fn generate_puzzle(&self, grid: &Grid, words: &[String], filename: &str) -> Result<(), String> {
        let (doc, page1, layer1) = PdfDocument::new(&self.title, Mm(PAGE_WIDTH_MM), Mm(PAGE_HEIGHT_MM), "Layer 1");
        let current_layer = doc.get_page(page1).get_layer(layer1);
        let font = doc.add_builtin_font(BuiltinFont::Courier).map_err(|e| e.to_string())?;
        let font_bold = doc.add_builtin_font(BuiltinFont::CourierBold).map_err(|e| e.to_string())?;

        self.draw_title(&current_layer, &font_bold);
        self.draw_grid(&current_layer, grid, &font, None);
        self.draw_word_list(&current_layer, grid, words, &font);

        let file = File::create(filename).map_err(|e| e.to_string())?;
        doc.save(&mut BufWriter::new(file)).map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn generate_answer_key(&self, grid: &Grid, placed_words: &[PlacementResult], filename: &str) -> Result<(), String> {
        let (doc, page1, layer1) = PdfDocument::new(&format!("{} - Gabarito", self.title), Mm(PAGE_WIDTH_MM), Mm(PAGE_HEIGHT_MM), "Layer 1");
        let current_layer = doc.get_page(page1).get_layer(layer1);
        let font = doc.add_builtin_font(BuiltinFont::Courier).map_err(|e| e.to_string())?;
        let font_bold = doc.add_builtin_font(BuiltinFont::CourierBold).map_err(|e| e.to_string())?;

        self.draw_title_answer(&current_layer, &font_bold);
        self.draw_grid(&current_layer, grid, &font, Some(placed_words));
        self.draw_placed_words_list(&current_layer, grid, placed_words, &font);

        let file = File::create(filename).map_err(|e| e.to_string())?;
        doc.save(&mut BufWriter::new(file)).map_err(|e| e.to_string())?;
        Ok(())
    }

    fn draw_title(&self, layer: &PdfLayerReference, font: &IndirectFontRef) {
        layer.use_text(&self.title, 18.0, Mm(MARGIN_MM), Mm(PAGE_HEIGHT_MM - MARGIN_MM), font);
    }

    fn draw_title_answer(&self, layer: &PdfLayerReference, font: &IndirectFontRef) {
        let title = format!("{} - GABARITO", self.title);
        layer.use_text(&title, 18.0, Mm(MARGIN_MM), Mm(PAGE_HEIGHT_MM - MARGIN_MM), font);
    }

    fn draw_grid(&self, layer: &PdfLayerReference, grid: &Grid, font: &IndirectFontRef, highlights: Option<&[PlacementResult]>) {
        let start_x = MARGIN_MM;
        let start_y = PAGE_HEIGHT_MM - MARGIN_MM - 15.0;
        let highlighted_cells = self.get_highlighted_cells(grid, highlights);

        for (row_idx, row) in grid.cells.iter().enumerate() {
            for (col_idx, cell) in row.iter().enumerate() {
                let x = start_x + (col_idx as f32 * CELL_SIZE_MM);
                let y = start_y - (row_idx as f32 * CELL_SIZE_MM);

                if highlighted_cells.contains(&(row_idx, col_idx)) {
                    self.draw_highlight(layer, x, y);
                }

                let ch = cell.unwrap_or(' ');
                layer.use_text(&ch.to_string(), 12.0, Mm(x + 2.5), Mm(y - 5.5), font);
            }
        }

        self.draw_grid_border(layer, grid.size, start_x, start_y);
    }

    fn get_highlighted_cells(&self, grid: &Grid, highlights: Option<&[PlacementResult]>) -> Vec<(usize, usize)> {
        let mut cells = Vec::new();
        if let Some(placements) = highlights {
            for placement in placements {
                let (dr, dc) = placement.direction.deltas();
                for i in 0..placement.word.len() {
                    let row = (placement.row as i32 + dr * i as i32) as usize;
                    let col = (placement.col as i32 + dc * i as i32) as usize;
                    if row < grid.size && col < grid.size {
                        cells.push((row, col));
                    }
                }
            }
        }
        cells
    }

    fn draw_highlight(&self, layer: &PdfLayerReference, x: f32, y: f32) {
        let rect = Rect::new(Mm(x), Mm(y - CELL_SIZE_MM + 1.0), Mm(x + CELL_SIZE_MM), Mm(y + 1.0));
        layer.set_fill_color(Color::Rgb(Rgb::new(1.0, 1.0, 0.0, None)));
        layer.add_rect(rect);
        layer.set_fill_color(Color::Rgb(Rgb::new(0.0, 0.0, 0.0, None)));
    }

    fn draw_grid_border(&self, layer: &PdfLayerReference, size: usize, start_x: f32, start_y: f32) {
        let grid_width = size as f32 * CELL_SIZE_MM;
        let grid_height = size as f32 * CELL_SIZE_MM;

        layer.set_outline_color(Color::Rgb(Rgb::new(0.0, 0.0, 0.0, None)));
        layer.set_outline_thickness(0.5);

        for i in 0..=size {
            let x = start_x + (i as f32 * CELL_SIZE_MM);
            let line = Line {
                points: vec![
                    (Point::new(Mm(x), Mm(start_y + 1.0)), false),
                    (Point::new(Mm(x), Mm(start_y - grid_height + 1.0)), false),
                ],
                is_closed: false,
            };
            layer.add_line(line);
        }

        for i in 0..=size {
            let y = start_y - (i as f32 * CELL_SIZE_MM) + 1.0;
            let line = Line {
                points: vec![
                    (Point::new(Mm(start_x), Mm(y)), false),
                    (Point::new(Mm(start_x + grid_width), Mm(y)), false),
                ],
                is_closed: false,
            };
            layer.add_line(line);
        }
    }

    fn draw_word_list(&self, layer: &PdfLayerReference, grid: &Grid, words: &[String], font: &IndirectFontRef) {
        let grid_height = grid.size as f32 * CELL_SIZE_MM;
        let start_y = PAGE_HEIGHT_MM - MARGIN_MM - 15.0 - grid_height - 15.0;
        let cols = 3;
        let col_width = (PAGE_WIDTH_MM - 2.0 * MARGIN_MM) / cols as f32;

        layer.use_text("Palavras para encontrar:", 10.0, Mm(MARGIN_MM), Mm(start_y), font);

        for (i, word) in words.iter().enumerate() {
            let col = i % cols;
            let row = i / cols;
            let x = MARGIN_MM + (col as f32 * col_width);
            let y = start_y - 8.0 - (row as f32 * 6.0);
            if y > MARGIN_MM {
                layer.use_text(word, 10.0, Mm(x), Mm(y), font);
            }
        }
    }

    fn draw_placed_words_list(&self, layer: &PdfLayerReference, grid: &Grid, placed_words: &[PlacementResult], font: &IndirectFontRef) {
        let grid_height = grid.size as f32 * CELL_SIZE_MM;
        let start_y = PAGE_HEIGHT_MM - MARGIN_MM - 15.0 - grid_height - 15.0;
        let cols = 3;
        let col_width = (PAGE_WIDTH_MM - 2.0 * MARGIN_MM) / cols as f32;

        layer.use_text("Palavras encontradas:", 10.0, Mm(MARGIN_MM), Mm(start_y), font);

        for (i, placement) in placed_words.iter().enumerate() {
            let col = i % cols;
            let row = i / cols;
            let x = MARGIN_MM + (col as f32 * col_width);
            let y = start_y - 8.0 - (row as f32 * 6.0);
            if y > MARGIN_MM {
                let text = format!("{} ({},{})", placement.word.original, placement.row, placement.col);
                layer.use_text(&text, 10.0, Mm(x), Mm(y), font);
            }
        }
    }
}
