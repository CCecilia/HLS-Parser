enum HdcpLevel {}

#[derive(Debug)]
pub struct VariantPlaylist {
    average_bandwidth: Option<u32>,
    bandwidth: u32,
    frame_rate: Option<f64>,
    hdcp_level: Option<String>,
    resolution: Option<(u32, u32)>,
    video_range: String,
    codecs: Vec<String>,
    uri: Option<String>,
}

impl VariantPlaylist {
    pub fn parse_from_tag_data(tag_data: &str) -> VariantPlaylist {
        let mut variant = VariantPlaylist {
            average_bandwidth: None,
            bandwidth: 0,
            frame_rate: None,
            hdcp_level: None,
            resolution: None,
            video_range: String::from("SDR"),
            codecs: vec![],
            uri: None,
        };
        fn parse(tag_data: &str, mut variant: VariantPlaylist) -> VariantPlaylist {
            let split = tag_data.split_once("=");

            match split {
                Some((attribute, rest)) => match attribute {
                    "CODECS" => {
                        let data_split = rest.split_once("\",");
                        match data_split {
                            Some((attr_data, remainder_to_parse)) => {
                                println!("CODECS attr data {}", attr_data);
                                let cleaned = attr_data.replace("\"", "");
                                variant.codecs = cleaned
                                    .split(",")
                                    .collect::<Vec<&str>>()
                                    .into_iter()
                                    .map(|s| s.to_string())
                                    .collect();
                                parse(remainder_to_parse, variant)
                            }
                            None => variant,
                        }
                    }
                    "BANDWIDTH" => {
                        let data_split = rest.split_once(",");
                        match data_split {
                            Some((attr_data, remainder_to_parse)) => {
                                let parse_result = attr_data.parse::<u32>();

                                if let Ok(bandwidth) = parse_result {
                                    println!("Bandwidth {}", &bandwidth);
                                    variant.bandwidth = bandwidth;
                                }
                                parse(remainder_to_parse, variant)
                            }
                            None => variant,
                        }
                    }

                    "AVERAGE-BANDWIDTH" => {
                        let data_split = rest.split_once(",");
                        match data_split {
                            Some((attr_data, remainder_to_parse)) => {
                                let parse_result = attr_data.parse::<u32>();

                                if let Ok(bandwidth) = parse_result {
                                    println!("AVERAGE-BANDWIDTH {}", &bandwidth);
                                    variant.average_bandwidth = Some(bandwidth);
                                }
                                parse(remainder_to_parse, variant)
                            }
                            None => variant,
                        }
                    }
                    "RESOLUTION" => {
                        let data_split = rest.split_once(",");
                        match data_split {
                            Some((attr_data, remainder_to_parse)) => {
                                let resolution_split = attr_data.split_once("x");
                                if let Some((width_str, height_str)) = resolution_split {
                                    let width_parse = width_str.parse::<u32>();
                                    let height_parse = height_str.parse::<u32>();

                                    if let Ok(width) = width_parse {
                                        if let Ok(height) = height_parse {
                                            variant.resolution = Some((width, height));
                                        }
                                    }
                                }
                                parse(remainder_to_parse, variant)
                            }
                            None => variant,
                        }
                    }
                    "FRAME-RATE" => {
                        let data_split = rest.split_once(",");
                        match data_split {
                            Some((attr_data, remainder_to_parse)) => {
                                let parse_result = attr_data.parse::<f64>();

                                if let Ok(frame_rate) = parse_result {
                                    println!("frame_rate {}", &frame_rate);
                                    variant.frame_rate = Some(frame_rate);
                                }
                                parse(remainder_to_parse, variant)
                            }
                            None => variant,
                        }
                    }
                    "VIDEO-RANGE" => {
                        let data_split = rest.split_once(",");
                        match data_split {
                            Some((attr_data, remainder_to_parse)) => {
                                variant.video_range = attr_data.to_owned();
                                parse(remainder_to_parse, variant)
                            }
                            None => variant,
                        }
                    }
                    "HDCP-LEVEL" => {
                        let data_split = rest.split_once(",");
                        match data_split {
                            Some((attr_data, remainder_to_parse)) => {
                                println!("HDCP-LEVEL {}", attr_data);
                                parse(remainder_to_parse, variant)
                            }
                            None => variant,
                        }
                    }
                    _ => variant,
                },
                None => return variant,
            }
        }

        variant = parse(tag_data, variant);

        variant
    }
}
