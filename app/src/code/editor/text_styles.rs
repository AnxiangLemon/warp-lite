use pathfinder_color::ColorU;
use warp_core::semantic_selection::SemanticSelection;
use warp_core::ui::builder::CHECK_SVG_PATH;
use warp_core::ui::theme::color::internal_colors;
use warp_editor::render::model::{
    BrokenLinkStyle, CheckBoxStyle, HorizontalRuleStyle, InlineCodeStyle,
    ParagraphStyles, RichTextStyles, TableStyle,
};
use warp_editor::selection::{TextDirection, TextUnit};
use warpui::elements::Border;
use warpui::{AppContext, SingletonEntity as _};

use crate::appearance::Appearance;
use crate::settings::{FontSettings, derived_notebook_font_size};
use crate::themes::theme::Fill;
use warpui::ui_components::checkbox::HOVER_BACKGROUND_COLOR;
use crate::util::color::{ContrastingColor, MinimumAllowedContrast};

const LINE_HEIGHT_RATIO: f32 = 1.5;
const BASELINE_RATIO: f32 = 0.7;

#[derive(Clone, Copy)]
pub struct MarkdownTableAppearance {
    pub border_color: ColorU,
    pub header_background: ColorU,
    pub cell_background: ColorU,
    pub alternate_row_background: Option<ColorU>,
    pub text_color: ColorU,
    pub header_text_color: ColorU,
    pub scrollbar_nonactive_thumb_color: ColorU,
    pub scrollbar_active_thumb_color: ColorU,
    pub cell_padding: f32,
    pub outer_border: bool,
    pub column_dividers: bool,
    pub row_dividers: bool,
}

fn markdown_table_style(
    appearance: &Appearance,
    font_family: warpui::fonts::FamilyId,
    font_size: f32,
) -> TableStyle {
    let theme = appearance.theme();
    TableStyle {
        border_color: warp_core::ui::theme::color::internal_colors::neutral_4(theme),
        header_background: ColorU::transparent_black(),
        cell_background: ColorU::transparent_black(),
        alternate_row_background: None,
        text_color: warp_core::ui::theme::color::internal_colors::text_sub(theme, theme.background()),
        header_text_color: warp_core::ui::theme::color::internal_colors::text_main(
            theme,
            theme.background(),
        ),
        scrollbar_nonactive_thumb_color: theme.nonactive_ui_detail().into_solid(),
        scrollbar_active_thumb_color: theme.active_ui_detail().into_solid(),
        font_family,
        font_size,
        cell_padding: 12.,
        outer_border: false,
        column_dividers: false,
        row_dividers: true,
    }
}

/// Build [`RichTextStyles`] based on the current [`Appearance`].
pub fn rich_text_styles(appearance: &Appearance, font_settings: &FontSettings) -> RichTextStyles {
    let theme = appearance.theme();
    let inline_font_color: ColorU = theme.terminal_colors().normal.red.into();
    let font_size = derived_notebook_font_size(font_settings);

    let base_text = ParagraphStyles {
        font_size,
        font_weight: Default::default(),
        line_height_ratio: LINE_HEIGHT_RATIO,
        font_family: appearance.ui_font_family(),
        text_color: theme.main_text_color(theme.background()).into_solid(),
        baseline_ratio: BASELINE_RATIO,
        fixed_width_tab_size: None,
    };

    RichTextStyles {
        base_text,
        code_text: ParagraphStyles {
            font_family: appearance.monospace_font_family(),
            font_size,
            font_weight: Default::default(),
            line_height_ratio: LINE_HEIGHT_RATIO,
            text_color: theme.main_text_color(theme.background()).into_solid(),
            baseline_ratio: BASELINE_RATIO,
            fixed_width_tab_size: Some(4),
        },
        code_background: theme.background().into(),
        embedding_background: theme.surface_2().into(),
        embedding_text: ParagraphStyles {
            font_size,
            font_weight: Default::default(),
            line_height_ratio: LINE_HEIGHT_RATIO,
            font_family: appearance.monospace_font_family(),
            text_color: theme.main_text_color(theme.surface_2()).into_solid(),
            baseline_ratio: BASELINE_RATIO,
            fixed_width_tab_size: Some(4),
        },
        code_border: Border::all(1.).with_border_fill(theme.surface_3()),
        placeholder_color: appearance
            .theme()
            .hint_text_color(theme.background())
            .into_solid(),
        selection_fill: appearance.theme().text_selection_color().into(),
        cursor_fill: theme
            .cursor()
            .on_background(theme.background(), MinimumAllowedContrast::Text)
            .into(),
        inline_code_style: InlineCodeStyle {
            font_family: appearance.monospace_font_family(),
            background: theme.surface_3().into(),
            font_color: inline_font_color
                .on_background(theme.surface_3().into(), MinimumAllowedContrast::Text),
        },
        check_box_style: CheckBoxStyle {
            border_color: theme.foreground().into(),
            border_width: 2.,
            icon_path: CHECK_SVG_PATH,
            background: theme.accent().into(),
            hover_background: *HOVER_BACKGROUND_COLOR,
        },
        horizontal_rule_style: HorizontalRuleStyle {
            color: theme.surface_3().into(),
            rule_height: 3.,
        },
        broken_link_style: BrokenLinkStyle {
            icon_path: "bundled/svg/link-broken-02.svg",
            icon_color: theme.terminal_colors().normal.red.into(),
        },
        block_spacings: Default::default(),
        show_placeholder_text_on_empty_block: true,
        minimum_paragraph_height: Some(base_text.line_height()),
        cursor_width: 3.,
        highlight_urls: true,
        table_style: markdown_table_style(appearance, appearance.ui_font_family(), font_size),
    }
}

/// Text unit for by-word navigation with the current word boundary policy.
pub fn word_unit(ctx: &AppContext) -> TextUnit {
    TextUnit::Word(SemanticSelection::as_ref(ctx).word_boundary_policy())
}

#[allow(dead_code)]
pub fn word_unit_for_direction(_direction: TextDirection, ctx: &AppContext) -> TextUnit {
    word_unit(ctx)
}

pub fn markdown_table_appearance(appearance: &Appearance) -> MarkdownTableAppearance {
    let theme = appearance.theme();
    MarkdownTableAppearance {
        border_color: internal_colors::neutral_4(theme),
        header_background: ColorU::transparent_black(),
        cell_background: ColorU::transparent_black(),
        alternate_row_background: None,
        text_color: internal_colors::text_sub(theme, theme.background()),
        header_text_color: internal_colors::text_main(theme, theme.background()),
        scrollbar_nonactive_thumb_color: theme.nonactive_ui_detail().into_solid(),
        scrollbar_active_thumb_color: theme.active_ui_detail().into_solid(),
        cell_padding: 12.,
        outer_border: false,
        column_dividers: false,
        row_dividers: true,
    }
}
