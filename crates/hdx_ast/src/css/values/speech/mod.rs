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
// pub enum VoiceVolume {}

// https://drafts.csswg.org/css-speech-1/#voice-balance
#[value(" <number> | left | center | right | leftwards | rightwards ")]
#[initial("center")]
#[applies_to("all elements")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("not animatable")]
pub enum VoiceBalance {}

// https://drafts.csswg.org/css-speech-1/#speak
#[value(" auto | never | always ")]
#[initial("auto")]
#[applies_to("all elements")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("not animatable")]
pub enum Speak {}

// // https://drafts.csswg.org/css-speech-1/#speak-as
// #[value(" normal | spell-out || digits || [ literal-punctuation | no-punctuation ] ")]
// #[initial("normal")]
// #[applies_to("all elements")]
// #[inherited("yes")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("not animatable")]
// pub enum SpeakAs {}

// https://drafts.csswg.org/css-speech-1/#pause-before
#[value(" <time [0s,∞]> | none | x-weak | weak | medium | strong | x-strong ")]
#[initial("none")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("not animatable")]
pub enum PauseBefore {}

// https://drafts.csswg.org/css-speech-1/#pause-after
#[value(" <time [0s,∞]> | none | x-weak | weak | medium | strong | x-strong ")]
#[initial("none")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("not animatable")]
pub enum PauseAfter {}

// https://drafts.csswg.org/css-speech-1/#pause
#[value(" <'pause-before'> <'pause-after'>? ")]
#[initial("N/A (see individual properties)")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("not animatable")]
pub struct Pause;

// https://drafts.csswg.org/css-speech-1/#rest-before
#[value(" <time [0s,∞]> | none | x-weak | weak | medium | strong | x-strong ")]
#[initial("none")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("not animatable")]
pub enum RestBefore {}

// https://drafts.csswg.org/css-speech-1/#rest-after
#[value(" <time [0s,∞]> | none | x-weak | weak | medium | strong | x-strong ")]
#[initial("none")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("not animatable")]
pub enum RestAfter {}

// https://drafts.csswg.org/css-speech-1/#rest
#[value(" <'rest-before'> <'rest-after'>? ")]
#[initial("N/A (see individual properties)")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("not animatable")]
pub struct Rest;

// // https://drafts.csswg.org/css-speech-1/#cue-before
// #[value(" <uri> <decibel>? | none ")]
// #[initial("none")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("not animatable")]
// pub enum CueBefore {}

// // https://drafts.csswg.org/css-speech-1/#cue-after
// #[value(" <uri> <decibel>? | none ")]
// #[initial("none")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("not animatable")]
// pub enum CueAfter {}

// // https://drafts.csswg.org/css-speech-1/#cue
// #[value(" <'cue-before'> <'cue-after'>? ")]
// #[initial("N/A (see individual properties)")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("not animatable")]
// pub struct Cue;

// // https://drafts.csswg.org/css-speech-1/#voice-family
// #[value(" [[<family-name> | <generic-voice>],]* [<family-name> | <generic-voice>] | preserve ")]
// #[initial("implementation-dependent")]
// #[applies_to("all elements")]
// #[inherited("yes")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("not animatable")]
// pub enum VoiceFamily {}

// // https://drafts.csswg.org/css-speech-1/#voice-rate
// #[value(" [normal | x-slow | slow | medium | fast | x-fast] || <percentage [0,∞]> ")]
// #[initial("normal")]
// #[applies_to("all elements")]
// #[inherited("yes")]
// #[percentages("refer to default value")]
// #[canonical_order("per grammar")]
// #[animation_type("not animatable")]
// pub enum VoiceRate {}

// // https://drafts.csswg.org/css-speech-1/#voice-pitch
// #[value(" <frequency [0Hz,∞]> && absolute | [[x-low | low | medium | high | x-high] || [<frequency> | <semitones> | <percentage>]] ")]
// #[initial("medium")]
// #[applies_to("all elements")]
// #[inherited("yes")]
// #[percentages("refer to inherited value")]
// #[canonical_order("per grammar")]
// #[animation_type("not animatable")]
// pub enum VoicePitch {}

// // https://drafts.csswg.org/css-speech-1/#voice-range
// #[value(" <frequency [0Hz,∞]> && absolute | [[x-low | low | medium | high | x-high] || [<frequency> | <semitones> | <percentage>]] ")]
// #[initial("medium")]
// #[applies_to("all elements")]
// #[inherited("yes")]
// #[percentages("refer to inherited value")]
// #[canonical_order("per grammar")]
// #[animation_type("not animatable")]
// pub enum VoiceRange {}

// https://drafts.csswg.org/css-speech-1/#voice-stress
#[value(" normal | strong | moderate | none | reduced ")]
#[initial("normal")]
#[applies_to("all elements")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("not animatable")]
pub enum VoiceStress {}

// https://drafts.csswg.org/css-speech-1/#voice-duration
#[value(" auto | <time [0s,∞]> ")]
#[initial("auto")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("not animatable")]
pub enum VoiceDuration {}
