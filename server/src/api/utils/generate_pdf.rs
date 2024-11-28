use axum::http::StatusCode;
use axum::response::Response;
use printpdf::*;

use crate::api::models::SoftwareReviewDTO;
use crate::api::ReviewOptions;
use crate::{Error, Result};

#[tracing::instrument(name = "generating pdf for software review", skip(software_review))]
pub async fn generate_pdf(software_review: &SoftwareReviewDTO) -> Result<Response> {
    let (doc, page, layer) = PdfDocument::new(
        software_review
            .software_request
            .software
            .software_name
            .clone(),
        Mm(210.0), // Width (A4 size)
        Mm(297.0), // Height (A4 size)
        "Layer 1",
    );

    let current_layer = doc.get_page(page).get_layer(layer);

    let image_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("assets")
        .join("img")
        .join("logo.bmp");

    let mut image_file = std::fs::File::open(image_path)
        .map_err(|err| Error::ServerError(std::sync::Arc::new(err.into())))?;

    let image = Image::try_from(
        image_crate::codecs::bmp::BmpDecoder::new(&mut image_file)
            .map_err(|err| Error::ServerError(std::sync::Arc::new(err.into())))?,
    )
    .map_err(|err| Error::ServerError(std::sync::Arc::new(err.into())))?;

    let padding = Mm(15.0);
    let default_color = Color::Rgb(Rgb::new(0.0, 0.0, 0.0, None));

    image.add_to_layer(
        current_layer.clone(),
        ImageTransform {
            translate_x: Some(padding),
            translate_y: Some(Mm(297.0 - 20.0) - padding),
            scale_x: Some(0.2),
            scale_y: Some(0.2),
            rotate: None,
            dpi: None,
        },
    );

    let font_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("assets")
        .join("font")
        .join("NotoSans-Regular.ttf");

    let font_symbol_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("assets")
        .join("font")
        .join("NotoSansSymbols2-Regular.ttf");

    let font = doc
        .add_external_font(
            std::fs::File::open(font_path)
                .map_err(|err| Error::ServerError(std::sync::Arc::new(err.into())))?,
        )
        .map_err(|err| Error::ServerError(std::sync::Arc::new(err.into())))?;

    let font_symbol = doc
        .add_external_font(
            std::fs::File::open(font_symbol_path)
                .map_err(|err| Error::ServerError(std::sync::Arc::new(err.into())))?,
        )
        .map_err(|err| Error::ServerError(std::sync::Arc::new(err.into())))?;

    // ----------------------------------------------------------------------------
    current_layer.begin_text_section();

    current_layer.set_font(&font, 16.0);
    current_layer.set_outline_thickness(1.2);
    current_layer.set_text_cursor(padding, Mm(280.0 - 20.0) - padding);
    current_layer.set_line_height(33.0);
    current_layer.set_word_spacing(0.0);
    current_layer.set_character_spacing(1.0);
    current_layer.set_text_rendering_mode(TextRenderingMode::FillStroke);

    current_layer.write_text("BITS Application Security Review", &font);
    current_layer.add_line_break();

    current_layer.set_font(&font, 11.0);
    current_layer.set_outline_thickness(0.0);
    current_layer.write_text(
        software_review
            .software_request
            .software
            .software_name
            .clone(),
        &font,
    );
    current_layer.add_line_break();
    current_layer.write_text(
        format!(
            "Request #{}",
            software_review.software_request.td_request_id
        ),
        &font,
    );
    current_layer.add_line_break();
    current_layer.write_text(
        format!(
            "Date: {}",
            software_review
                .created_at
                .unwrap_or_default()
                .naive_local()
                .format("%m/%d/%Y")
        ),
        &font,
    );
    current_layer.add_line_break();
    current_layer.write_text(
        format!("Reviewer Name: {}", software_review.reviewer.name.clone(),),
        &font,
    );

    current_layer.add_line_break();
    current_layer.add_line_break();
    // ----------------------------------------------------------------------------
    current_layer.set_font(&font, 14.0);
    current_layer.set_outline_thickness(1.2);
    current_layer.write_text("Installation Criteria", &font);
    current_layer.add_line_break();

    current_layer.set_outline_thickness(0.0);

    current_layer.set_font(&font_symbol, 15.0);
    let (glyph, color) = convert_response_to_glyph(software_review.is_supported.clone());
    current_layer.set_fill_color(color);
    current_layer.write_text(glyph, &font_symbol);
    current_layer.set_font(&font, 11.0);
    current_layer.set_fill_color(default_color.clone());
    current_layer.write_text(format!(" | {}", "Still supported by developer"), &font);
    current_layer.add_line_break();

    current_layer.set_font(&font_symbol, 15.0);
    let (glyph, color) = convert_response_to_glyph(software_review.is_current_version.clone());
    current_layer.set_fill_color(color);
    current_layer.write_text(glyph, &font_symbol);
    current_layer.set_font(&font, 11.0);
    current_layer.set_fill_color(default_color.clone());
    current_layer.write_text(format!(" | {}", "Current version is requested"), &font);
    current_layer.add_line_break();

    current_layer.set_font(&font_symbol, 15.0);
    let (glyph, color) = convert_response_to_glyph(software_review.is_reputation_good.clone());
    current_layer.set_fill_color(color);
    current_layer.write_text(glyph, &font_symbol);
    current_layer.set_font(&font, 11.0);
    current_layer.set_fill_color(default_color.clone());
    current_layer.write_text(format!(" | {}", "Developer reputation is good"), &font);
    current_layer.add_line_break();

    current_layer.set_font(&font_symbol, 15.0);
    let (glyph, color) =
        convert_response_to_glyph(software_review.is_installation_from_developer.clone());
    current_layer.set_fill_color(color);
    current_layer.write_text(glyph, &font_symbol);
    current_layer.set_font(&font, 11.0);
    current_layer.set_fill_color(default_color.clone());
    current_layer.write_text(
        format!(
            " | {}",
            "Installation package received from developer/vendor"
        ),
        &font,
    );
    current_layer.add_line_break();

    current_layer.set_font(&font_symbol, 15.0);
    let (glyph, color) = convert_response_to_glyph(software_review.is_local_admin_required.clone());
    current_layer.set_fill_color(color);
    current_layer.write_text(glyph, &font_symbol);
    current_layer.set_font(&font, 11.0);
    current_layer.set_fill_color(default_color.clone());
    current_layer.write_text(
        format!(" | {}", "Local administrator not required for daily use"),
        &font,
    );
    current_layer.add_line_break();

    current_layer.set_font(&font_symbol, 15.0);
    let (glyph, color) =
        convert_response_to_glyph(software_review.is_connected_to_brockport_cloud.clone());
    current_layer.set_fill_color(color);
    current_layer.write_text(glyph, &font_symbol);
    current_layer.set_font(&font, 11.0);
    current_layer.set_fill_color(default_color.clone());
    current_layer.write_text(
        format!(" | {}", "Doesn't connect to SUNY Brockport cloud accounts"),
        &font,
    );
    current_layer.add_line_break();

    current_layer.set_font(&font_symbol, 15.0);
    let (glyph, color) = convert_response_to_glyph(
        software_review
            .is_connected_to_cloud_services_or_client
            .clone(),
    );
    current_layer.set_fill_color(color);
    current_layer.write_text(glyph, &font_symbol);
    current_layer.set_font(&font, 11.0);
    current_layer.set_fill_color(default_color.clone());
    current_layer.write_text(
        format!(
            " | {}",
            "Doesn't connect to any other cloud services or serve as a client for cloud services"
        ),
        &font,
    );
    current_layer.add_line_break();

    current_layer.set_font(&font_symbol, 15.0);
    let (glyph, color) =
        convert_response_to_glyph(software_review.is_security_or_optimization_software.clone());
    current_layer.set_fill_color(color);
    current_layer.write_text(glyph, &font_symbol);
    current_layer.set_font(&font, 11.0);
    current_layer.set_fill_color(default_color.clone());
    current_layer.write_text(
        format!(
            " | {}",
            "Isn't computer security software or optimization software"
        ),
        &font,
    );
    current_layer.add_line_break();

    current_layer.set_font(&font_symbol, 15.0);
    let (glyph, color) =
        convert_response_to_glyph(software_review.is_supported_by_current_os.clone());
    current_layer.set_fill_color(color);
    current_layer.write_text(glyph, &font_symbol);
    current_layer.set_font(&font, 11.0);
    current_layer.set_fill_color(default_color.clone());
    current_layer.write_text(
        format!(
            " | {}",
            "Supports the current operating systems deployed on campus"
        ),
        &font,
    );
    current_layer.add_line_break();
    current_layer.add_line_break();
    // ----------------------------------------------------------------------------
    current_layer.set_font(&font, 14.0);
    current_layer.set_outline_thickness(1.2);
    current_layer.write_text("Notes", &font);
    current_layer.add_line_break();

    current_layer.set_font(&font, 11.0);
    current_layer.set_outline_thickness(0.0);

    let review_notes = software_review.review_notes.clone().unwrap_or_default();
    let lines = split_into_lines(&review_notes, 80);

    for line in lines {
        current_layer.write_text(&line, &font);
        current_layer.add_line_break();
    }

    current_layer.end_text_section();
    // ----------------------------------------------------------------------------

    let mut buffer = Vec::new();
    doc.save(&mut std::io::BufWriter::new(&mut buffer))
        .map_err(|err| Error::ServerError(std::sync::Arc::new(err.into())))?;

    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/pdf")
        .header(
            "Content-Disposition",
            format!(
                "attachment; filename=\"{}.pdf\"",
                software_review.software_request.software.software_name
            ),
        )
        .body(buffer.into())
        .map_err(|err| Error::ServerError(std::sync::Arc::new(err.into())))?;

    Ok(response)
}

fn convert_response_to_glyph(review_response: ReviewOptions) -> (String, Color) {
    match review_response {
        ReviewOptions::TRUE => ("✓".into(), Color::Rgb(Rgb::new(0.0, 255.0, 0.0, None))),
        ReviewOptions::FALSE => ("✖".into(), Color::Rgb(Rgb::new(255.0, 0.0, 0.0, None))),
        // `?` won't render colors other than black or white when targeting fill
        _ => ("❓".into(), Color::Rgb(Rgb::new(5.0, 5.0, 5.0, None))),
    }
}

fn split_into_lines(text: &str, max_line_length: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current_line = String::new();

    for word in text.split_whitespace() {
        if current_line.len() + word.len() + 1 > max_line_length {
            lines.push(current_line.clone());
            current_line = word.to_string();
        } else {
            if !current_line.is_empty() {
                current_line.push(' ');
            }
            current_line.push_str(word);
        }
    }

    if !current_line.is_empty() {
        lines.push(current_line);
    }

    lines
}
