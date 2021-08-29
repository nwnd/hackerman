use super::CommandHandlerError;
use image::DynamicImage;

pub async fn handle_fakepng(url: String) -> Result<String, CommandHandlerError> {
    // Execute a rippl pipeline on the URL
    let url = url::Url::parse(&url)?;
    let new_url =
        rippl::process_remote_image(&url, |img| Some(DynamicImage::ImageRgba8(falsepng::falsify_png(&img)))).await?;

    Ok(format!("{}", new_url))
}
