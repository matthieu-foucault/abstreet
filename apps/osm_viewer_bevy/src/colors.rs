use bevy::prelude::Color;
use map_model::osm::RoadRank;
use map_model::{LaneType, Map};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ColorSchemeChoice {
    DayMode,
    NightMode,
    Textured,
    ClassicDayMode,
}
pub struct ColorScheme {
    scheme: ColorSchemeChoice,

    pub road_outlines: bool,
    pub road_class_colors: bool,
    pub show_buildings_in_minimap: bool,

    // UI
    // pub panel_bg: Color,
    // pub inner_panel_bg: Color,
    // pub day_time_slider: Color,
    // pub night_time_slider: Color,
    // pub selected: Color,
    // pub current_object: Color,
    // pub perma_selected_object: Color,
    // pub fade_map_dark: Color,
    // gui_style: Style,
    // pub minimap_cursor_border: Color,
    // pub minimap_cursor_bg: Option<Color>,

    // Roads
    driving_lane: Color,
    bus_lane: Color,
    parking_lane: Color,
    bike_lane: Color,
    sidewalk: Color,
    pub sidewalk_lines: Color,
    pub general_road_marking: Color,
    road_center_line: Color,
    pub light_rail_track: Color,
    pub private_road: Option<Color>,
    pub unzoomed_highway: Color,
    pub unzoomed_arterial: Color,
    pub unzoomed_residential: Color,
    pub unzoomed_cycleway: Color,
    pub unzoomed_footway: Color,
    footway: Color,
    shared_use: Color,

    // Intersections
    pub normal_intersection: Color,
    pub stop_sign: Color,
    pub stop_sign_pole: Color,
    pub signal_protected_turn: Color,
    pub signal_permitted_turn: Color,
    pub signal_banned_turn: Color,
    pub signal_box: Color,
    pub signal_spinner: Color,
    pub signal_turn_block_bg: Color,

    // Problems encountered on a trip
    pub slowest_intersection: Color,
    pub slower_intersection: Color,
    pub slow_intersection: Color,

    // Other static elements
    pub void_background: Color,
    pub map_background: Color,
    pub unzoomed_interesting_intersection: Color,
    pub residential_building: Color,
    pub commercial_building: Color,
    pub building_outline: Color,
    pub parking_lot: Color,
    pub grass: Color,
    pub water: Color,
    pub study_area: Color,

    // Unzoomed dynamic elements
    pub unzoomed_car: Color,
    pub unzoomed_bike: Color,
    pub unzoomed_bus: Color,
    pub unzoomed_pedestrian: Color,

    // Agents
    agent_colors: Vec<Color>,
    pub route: Color,
    pub turn_arrow: Color,
    pub brake_light: Color,
    pub bus_body: Color,
    pub bus_label: Color,
    pub train_body: Color,
    pub ped_head: Color,
    pub ped_foot: Color,
    pub ped_preparing_bike_body: Color,
    pub ped_crowd: Color,
    pub bike_frame: Color,
    pub parked_car: Color,

    // Layers
    // pub good_to_bad_red: ColorScale,
    // pub good_to_bad_green: ColorScale,
    pub bus_layer: Color,
    pub edits_layer: Color,

    // Misc
    pub parking_trip: Color,
    pub bike_trip: Color,
    pub bus_trip: Color,
    pub before_changes: Color,
    pub after_changes: Color,
}

impl ColorScheme {
    pub fn new(scheme: ColorSchemeChoice) -> ColorScheme {
        let mut cs = match scheme {
            ColorSchemeChoice::DayMode => ColorScheme::day_mode(),
            ColorSchemeChoice::NightMode => ColorScheme::night_mode(),
            ColorSchemeChoice::Textured => ColorScheme::textured(),
            ColorSchemeChoice::ClassicDayMode => ColorScheme::classic(),
        };
        cs.scheme = scheme;
        cs
    }

    fn classic() -> ColorScheme {
        let mut cs = Self::light_background();
        cs.scheme = ColorSchemeChoice::ClassicDayMode;
        cs
    }

    fn light_background() -> ColorScheme {
        ColorScheme {
            scheme: ColorSchemeChoice::DayMode,

            road_outlines: false,
            road_class_colors: false,
            show_buildings_in_minimap: true,

            // UI
            // panel_bg: gui_style.panel_bg,
            // inner_panel_bg: gui_style.section_bg,
            // day_time_slider: hex("F4DA22"),
            // night_time_slider: hex("12409D"),
            // selected: Color::RED.alpha(0.7),
            // current_object: Color::WHITE,
            // perma_selected_object: Color::BLUE,
            // fade_map_dark: Color::BLACK.alpha(0.6),
            // minimap_cursor_border: Color::BLACK,
            // minimap_cursor_bg: None,
            // gui_style,

            // Roads
            driving_lane: Color::BLACK,
            bus_lane: Color::rgb_u8(190, 74, 76),
            parking_lane: grey(0.2),
            bike_lane: Color::rgb_u8(15, 125, 75),
            sidewalk: grey(0.8),
            sidewalk_lines: grey(0.7),
            general_road_marking: Color::WHITE,
            road_center_line: Color::YELLOW,
            light_rail_track: hex("844204"),
            private_road: Some(hex("F0B0C0")),
            unzoomed_highway: hex("E892A2"),
            unzoomed_arterial: hex("FFC73E"),
            unzoomed_residential: Color::WHITE,
            unzoomed_cycleway: hex("0F7D4B"),
            unzoomed_footway: hex("DED68A"),
            // TODO Distinguish shared use and footway unzoomed or zoomed?
            footway: hex("DED68A"),
            shared_use: hex("DED68A"),

            // Intersections
            normal_intersection: grey(0.2),
            stop_sign: Color::RED,
            stop_sign_pole: grey(0.5),
            signal_protected_turn: hex("72CE36"),
            signal_permitted_turn: hex("4CA7E9"),
            signal_banned_turn: hex("EB3223"),
            signal_box: grey(0.5),
            signal_spinner: hex("F2994A"),
            signal_turn_block_bg: grey(0.6),

            // Problems encountered on a trip
            slowest_intersection: Color::RED,
            slower_intersection: Color::YELLOW,
            slow_intersection: Color::GREEN,

            // Other static elements
            void_background: Color::BLACK,
            map_background: grey(0.87),
            unzoomed_interesting_intersection: Color::BLACK,
            residential_building: hex("C4C1BC"),
            commercial_building: hex("9FABA7"),
            building_outline: hex("938E85"),
            parking_lot: grey(0.7),
            grass: hex("94C84A"),
            water: hex("A4C8EA"),
            study_area: hex("96830C"),

            // Unzoomed dynamic elements
            unzoomed_car: hex("FE5f55"),
            unzoomed_bike: hex("90BE6D"),
            unzoomed_bus: hex("FFD166"),
            unzoomed_pedestrian: hex("457B9D"),

            // Agents
            agent_colors: vec![
                hex("5C45A0"),
                hex("3E8BC3"),
                hex("E1BA13"),
                hex("96322F"),
                hex("00A27B"),
            ],
            route: Color::rgba(1.0, 0.65, 0.0, 0.5),
            turn_arrow: hex("DF8C3D"),
            brake_light: hex("FF1300"),
            bus_body: Color::rgb_u8(50, 133, 117),
            bus_label: Color::rgb_u8(249, 206, 24),
            train_body: hex("42B6E9"),
            ped_head: Color::rgb_u8(139, 69, 19),
            ped_foot: Color::BLACK,
            ped_preparing_bike_body: Color::rgb_u8(255, 0, 144),
            ped_crowd: Color::rgb(0.2, 0.7, 0.7),
            bike_frame: hex("AAA9AD"),
            parked_car: hex("938E85"),

            // Layers
            // good_to_bad_red: ColorScale(vec![hex("F19A93"), hex("A32015")]),
            // good_to_bad_green: ColorScale(vec![hex("BEDB92"), hex("397A4C")]),
            bus_layer: hex("4CA7E9"),
            edits_layer: hex("12409D"),

            // Misc
            parking_trip: hex("4E30A6"),
            bike_trip: Color::rgb_u8(15, 125, 75),
            bus_trip: Color::rgb_u8(190, 74, 76),
            before_changes: Color::BLUE,
            after_changes: Color::RED,
        }
    }

    // Shamelessly adapted from https://github.com/Uriopass/Egregoria
    fn night_mode() -> ColorScheme {
        let mut cs = ColorScheme::classic();
        cs.scheme = ColorSchemeChoice::NightMode;
        // cs.gui_style = widgetry::Style::dark_bg();

        cs.void_background = hex("200A24");
        cs.map_background = Color::BLACK;
        cs.grass = hex("243A1F");
        cs.water = hex("21374E");
        cs.residential_building = hex("2C422E");
        cs.commercial_building = hex("5D5F97");

        cs.driving_lane = hex("404040");
        cs.parking_lane = hex("353535");
        cs.sidewalk = hex("6B6B6B");
        cs.general_road_marking = hex("B1B1B1");
        cs.normal_intersection = cs.driving_lane;
        cs.road_center_line = cs.general_road_marking;

        cs.parking_lot = cs.sidewalk;
        cs.unzoomed_highway = cs.parking_lane;
        cs.unzoomed_arterial = cs.sidewalk;
        cs.unzoomed_residential = cs.driving_lane;
        cs.unzoomed_interesting_intersection = cs.unzoomed_highway;
        cs.stop_sign = hex("A32015");
        cs.private_road = Some(hex("9E757F"));
        cs.study_area = hex("D9B002");

        // cs.panel_bg = cs.gui_style.panel_bg;
        // cs.inner_panel_bg = cs.panel_bg.alpha(1.0);
        // cs.minimap_cursor_border = Color::WHITE;
        // cs.minimap_cursor_bg = Some(Color::rgba(238, 112, 46, 0.2));

        cs
    }

    fn textured() -> ColorScheme {
        let mut cs = ColorScheme::day_mode();
        cs.scheme = ColorSchemeChoice::Textured;
        // cs.grass = Texture::GRASS.into();
        // cs.water = Texture::STILL_WATER.into();
        // cs.map_background = Texture::CONCRETE.into();
        cs
    }

    fn day_mode() -> ColorScheme {
        // let mut cs = Self::light_background(Style::light_bg());
        let mut cs = Self::light_background();

        cs.scheme = ColorSchemeChoice::DayMode;
        cs.road_outlines = true;
        cs.road_class_colors = true;
        cs.show_buildings_in_minimap = false;

        cs.map_background = hex("EEE5C8");
        cs.grass = hex("BED4A3");
        cs.water = hex("6384D6");

        cs.sidewalk = hex("A9A9A9");
        cs.sidewalk_lines = hex("989898");

        cs.unzoomed_arterial = hex("F6A483");

        cs.residential_building = hex("C5D2E5");
        cs.commercial_building = hex("99AECC");

        cs
    }
}

impl ColorScheme {
    pub fn rotating_color_plot(&self, idx: usize) -> Color {
        modulo_color(
            &[
                Color::RED,
                Color::BLUE,
                Color::GREEN,
                Color::PURPLE,
                Color::BLACK,
            ],
            idx,
        )
    }

    pub fn rotating_color_agents(&self, idx: usize) -> Color {
        modulo_color(&self.agent_colors, idx)
    }

    pub fn unzoomed_road_surface(&self, rank: RoadRank) -> Color {
        match rank {
            RoadRank::Highway => self.unzoomed_highway,
            RoadRank::Arterial => self.unzoomed_arterial,
            RoadRank::Local => self.unzoomed_residential,
        }
    }

    pub fn zoomed_road_surface(&self, lane: LaneType, rank: RoadRank) -> Color {
        let main_asphalt = if self.road_class_colors {
            match rank {
                RoadRank::Highway => grey(0.3),
                RoadRank::Arterial => grey(0.4),
                RoadRank::Local => grey(0.5),
            }
        } else {
            self.driving_lane
        };
        let parking_asphalt = if self.road_class_colors {
            main_asphalt
        } else {
            self.parking_lane
        };

        match lane {
            LaneType::Driving => main_asphalt,
            LaneType::Bus => self.bus_lane,
            LaneType::Parking => parking_asphalt,
            LaneType::Sidewalk | LaneType::Shoulder => self.sidewalk,
            LaneType::Biking => self.bike_lane,
            LaneType::SharedLeftTurn => main_asphalt,
            LaneType::Construction => parking_asphalt,
            LaneType::LightRail => unreachable!(),
            LaneType::Buffer(_) => main_asphalt,
            LaneType::Footway => self.footway,
            LaneType::SharedUse => self.shared_use,
        }
    }
    pub fn zoomed_intersection_surface(&self, rank: RoadRank) -> Color {
        if self.road_class_colors {
            self.zoomed_road_surface(LaneType::Driving, rank)
        } else {
            self.normal_intersection
        }
    }

    pub fn curb(&self, rank: RoadRank) -> Color {
        // The curb should be darker than the asphalt to stand out
        match rank {
            RoadRank::Highway => grey(0.2),
            RoadRank::Arterial => grey(0.3),
            RoadRank::Local => grey(0.4),
        }
    }

    pub fn road_center_line(&self, map: &Map) -> Color {
        // TODO A more robust approach is to offload this question to osm2lanes, and color by
        // separators
        if map.get_name().city.country == "gb" {
            self.general_road_marking
        } else {
            self.road_center_line
        }
    }
}

fn modulo_color(colors: &[Color], idx: usize) -> Color {
    colors[idx % colors.len()]
}

// Convenience
fn hex(x: &str) -> Color {
    Color::hex(x).unwrap()
}

fn grey(f: f32) -> Color {
    Color::rgb(f, f, f)
}
