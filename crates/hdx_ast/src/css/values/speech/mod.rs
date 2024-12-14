mod impls;
pub mod types;

use impls::*;

/*
 * https://drafts.csswg.org/css-speech-1/
 * CSS Speech Module Level 1
 */

// // https://drafts.csswg.org/css-speech-1/#voice-volume
// #[value(" silent | [[x-soft | soft | medium | loud | x-loud] || <decibel>] ")]
// #[initial("medium")]
// #[applies_to("all elements")]
// #[inherited("yes")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("not animatable")]
// pub enum VoiceVolumeStyleValue {}

// https://drafts.csswg.org/css-speech-1/#voice-balance
#[value(" <number> | left | center | right | leftwards | rightwards ")]
#[initial("center")]
#[applies_to("all elements")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("not animatable")]
pub enum VoiceBalanceStyleValue {}

// https://drafts.csswg.org/css-speech-1/#speak
#[value(" auto | never | always ")]
#[initial("auto")]
#[applies_to("all elements")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("not animatable")]
pub enum SpeakStyleValue {}

// // https://drafts.csswg.org/css-speech-1/#speak-as
// #[value(" normal | spell-out || digits || [ literal-punctuation | no-punctuation ] ")]
// #[initial("normal")]
// #[applies_to("all elements")]
// #[inherited("yes")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("not animatable")]
// pub enum SpeakAsStyleValue {}

// https://drafts.csswg.org/css-speech-1/#pause-before
#[value(" <time [0s,∞]> | none | x-weak | weak | medium | strong | x-strong ")]
#[initial("none")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("not animatable")]
pub enum PauseBeforeStyleValue {}

// https://drafts.csswg.org/css-speech-1/#pause-after
#[value(" <time [0s,∞]> | none | x-weak | weak | medium | strong | x-strong ")]
#[initial("none")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("not animatable")]
pub enum PauseAfterStyleValue {}

// https://drafts.csswg.org/css-speech-1/#pause
#[value(" <'pause-before'> <'pause-after'>? ")]
#[initial("see individual properties")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("not animatable")]
pub struct PauseStyleValue;

// https://drafts.csswg.org/css-speech-1/#rest-before
#[value(" <time [0s,∞]> | none | x-weak | weak | medium | strong | x-strong ")]
#[initial("none")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("not animatable")]
pub enum RestBeforeStyleValue {}

// https://drafts.csswg.org/css-speech-1/#rest-after
#[value(" <time [0s,∞]> | none | x-weak | weak | medium | strong | x-strong ")]
#[initial("none")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("not animatable")]
pub enum RestAfterStyleValue {}

// https://drafts.csswg.org/css-speech-1/#rest
#[value(" <'rest-before'> <'rest-after'>? ")]
#[initial("see individual properties")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("not animatable")]
pub struct RestStyleValue;

// // https://drafts.csswg.org/css-speech-1/#cue-before
// #[value(" <uri> <decibel>? | none ")]
// #[initial("none")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("not animatable")]
// pub enum CueBeforeStyleValue {}

// // https://drafts.csswg.org/css-speech-1/#cue-after
// #[value(" <uri> <decibel>? | none ")]
// #[initial("none")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("not animatable")]
// pub enum CueAfterStyleValue {}

// // https://drafts.csswg.org/css-speech-1/#cue
// #[value(" <'cue-before'> <'cue-after'>? ")]
// #[initial("see individual properties")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("not animatable")]
// pub struct CueStyleValue;

// // https://drafts.csswg.org/css-speech-1/#voice-family
// #[value(" [[<family-name> | <generic-voice>],]* [<family-name> | <generic-voice>] | preserve ")]
// #[initial("implementation-dependent")]
// #[applies_to("all elements")]
// #[inherited("yes")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("not animatable")]
// pub enum VoiceFamilyStyleValue {}

// // https://drafts.csswg.org/css-speech-1/#voice-rate
// #[value(" [normal | x-slow | slow | medium | fast | x-fast] || <percentage [0,∞]> ")]
// #[initial("normal")]
// #[applies_to("all elements")]
// #[inherited("yes")]
// #[percentages("refer to default value")]
// #[canonical_order("per grammar")]
// #[animation_type("not animatable")]
// pub enum VoiceRateStyleValue {}

// // https://drafts.csswg.org/css-speech-1/#voice-pitch
// #[value(" <frequency [0Hz,∞]> && absolute | [[x-low | low | medium | high | x-high] || [<frequency> | <semitones> | <percentage>]] ")]
// #[initial("medium")]
// #[applies_to("all elements")]
// #[inherited("yes")]
// #[percentages("refer to inherited value")]
// #[canonical_order("per grammar")]
// #[animation_type("not animatable")]
// pub enum VoicePitchStyleValue {}

// // https://drafts.csswg.org/css-speech-1/#voice-range
// #[value(" <frequency [0Hz,∞]> && absolute | [[x-low | low | medium | high | x-high] || [<frequency> | <semitones> | <percentage>]] ")]
// #[initial("medium")]
// #[applies_to("all elements")]
// #[inherited("yes")]
// #[percentages("refer to inherited value")]
// #[canonical_order("per grammar")]
// #[animation_type("not animatable")]
// pub enum VoiceRangeStyleValue {}

// https://drafts.csswg.org/css-speech-1/#voice-stress
#[value(" normal | strong | moderate | none | reduced ")]
#[initial("normal")]
#[applies_to("all elements")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("not animatable")]
pub enum VoiceStressStyleValue {}

// https://drafts.csswg.org/css-speech-1/#voice-duration
#[value(" auto | <time [0s,∞]> ")]
#[initial("auto")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("not animatable")]
pub enum VoiceDurationStyleValue {}
