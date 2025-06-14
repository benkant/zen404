-- Schema for audio_narrative_database.sqlite

-- -----------------------------------------------------
-- Table `Artists`
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS `Artists` (
  `artist_id` INTEGER PRIMARY KEY AUTOINCREMENT,
  `name` TEXT NOT NULL UNIQUE,
  `sort_name` TEXT NULL
);

-- -----------------------------------------------------
-- Table `Albums`
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS `Albums` (
  `album_id` INTEGER PRIMARY KEY AUTOINCREMENT,
  `title` TEXT NOT NULL,
  `artist_id` INTEGER NOT NULL,
  `release_year` INTEGER NULL,
  `album_art_filepath` TEXT NULL, -- Relative path
  FOREIGN KEY (`artist_id`) REFERENCES `Artists` (`artist_id`)
    ON DELETE CASCADE ON UPDATE NO ACTION
);
CREATE INDEX IF NOT EXISTS `Albums_artist_id_idx` ON `Albums` (`artist_id` ASC);

-- -----------------------------------------------------
-- Table `Tracks`
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS `Tracks` (
  `track_id` INTEGER PRIMARY KEY AUTOINCREMENT,
  `title` TEXT NOT NULL,
  `artist_id` INTEGER NOT NULL,
  `album_id` INTEGER NULL,
  `project_track_id` TEXT UNIQUE NULL, -- For 'M101', 'F001' style IDs
  `bpm` REAL NULL,
  `key` TEXT NULL,
  `genre` TEXT NULL,
  `year` INTEGER NULL,
  `duration_ms` INTEGER NULL,
  `source_audio_filepath` TEXT UNIQUE NULL, -- Relative path
  `rating` INTEGER NULL, -- 1-5
  `energy_level` INTEGER NULL, -- 1-10
  `valence_level` INTEGER NULL, -- 1-10
  `narrative_sound_description` TEXT NULL,
  `is_cover` BOOLEAN NOT NULL DEFAULT 0,
  `original_artist_id` INTEGER NULL,
  FOREIGN KEY (`artist_id`) REFERENCES `Artists` (`artist_id`)
    ON DELETE CASCADE ON UPDATE NO ACTION,
  FOREIGN KEY (`album_id`) REFERENCES `Albums` (`album_id`)
    ON DELETE SET NULL ON UPDATE NO ACTION,
  FOREIGN KEY (`original_artist_id`) REFERENCES `Artists` (`artist_id`)
    ON DELETE SET NULL ON UPDATE NO ACTION
);
CREATE INDEX IF NOT EXISTS `Tracks_artist_id_idx` ON `Tracks` (`artist_id` ASC);
CREATE INDEX IF NOT EXISTS `Tracks_album_id_idx` ON `Tracks` (`album_id` ASC);
CREATE INDEX IF NOT EXISTS `Tracks_original_artist_id_idx` ON `Tracks` (`original_artist_id` ASC);

-- -----------------------------------------------------
-- Table `Stem_Types`
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS `Stem_Types` (
  `stem_type_id` INTEGER PRIMARY KEY AUTOINCREMENT,
  `name` TEXT NOT NULL UNIQUE
);

-- -----------------------------------------------------
-- Table `Stems`
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS `Stems` (
  `stem_id` INTEGER PRIMARY KEY AUTOINCREMENT,
  `track_id` INTEGER NOT NULL,
  `stem_type_id` INTEGER NOT NULL,
  `stem_filepath` TEXT NOT NULL UNIQUE, -- Relative path
  `duration_ms` INTEGER NULL,
  `processing_notes` TEXT NULL,
  FOREIGN KEY (`track_id`) REFERENCES `Tracks` (`track_id`)
    ON DELETE CASCADE ON UPDATE NO ACTION,
  FOREIGN KEY (`stem_type_id`) REFERENCES `Stem_Types` (`stem_type_id`)
    ON DELETE RESTRICT ON UPDATE NO ACTION,
  UNIQUE (`track_id`, `stem_type_id`)
);
CREATE INDEX IF NOT EXISTS `Stems_track_id_idx` ON `Stems` (`track_id` ASC);
CREATE INDEX IF NOT EXISTS `Stems_stem_type_id_idx` ON `Stems` (`stem_type_id` ASC);

-- -----------------------------------------------------
-- Table `Narrative_Frameworks`
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS `Narrative_Frameworks` (
  `framework_id` INTEGER PRIMARY KEY AUTOINCREMENT,
  `name` TEXT NOT NULL UNIQUE,
  `description` TEXT NULL
);

-- -----------------------------------------------------
-- Table `Narrative_Steps`
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS `Narrative_Steps` (
  `step_id` INTEGER PRIMARY KEY AUTOINCREMENT,
  `framework_id` INTEGER NOT NULL,
  `name` TEXT NOT NULL, -- e.g., "YOU", "NEED"
  `step_order` INTEGER NOT NULL, -- 1, 2, 3...
  `description` TEXT NULL,
  FOREIGN KEY (`framework_id`) REFERENCES `Narrative_Frameworks` (`framework_id`)
    ON DELETE CASCADE ON UPDATE NO ACTION,
  UNIQUE (`framework_id`, `name`),
  UNIQUE (`framework_id`, `step_order`)
);
CREATE INDEX IF NOT EXISTS `Narrative_Steps_framework_id_idx` ON `Narrative_Steps` (`framework_id` ASC);

-- -----------------------------------------------------
-- Table `Track_Narrative_Assignments`
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS `Track_Narrative_Assignments` (
  `assignment_id` INTEGER PRIMARY KEY AUTOINCREMENT,
  `track_id` INTEGER NOT NULL,
  `narrative_step_id` INTEGER NOT NULL,
  `assignment_type` TEXT NOT NULL, -- e.g., "Lad's Tackle", "Girl Parts"
  `role_in_step` TEXT NULL, -- e.g., "Primary Theme", "Counterpoint"
  FOREIGN KEY (`track_id`) REFERENCES `Tracks` (`track_id`)
    ON DELETE CASCADE ON UPDATE NO ACTION,
  FOREIGN KEY (`narrative_step_id`) REFERENCES `Narrative_Steps` (`step_id`)
    ON DELETE CASCADE ON UPDATE NO ACTION,
  UNIQUE (`track_id`, `narrative_step_id`, `assignment_type`)
);
CREATE INDEX IF NOT EXISTS `Track_Narrative_Assignments_track_id_idx` ON `Track_Narrative_Assignments` (`track_id` ASC);
CREATE INDEX IF NOT EXISTS `Track_Narrative_Assignments_narrative_step_id_idx` ON `Track_Narrative_Assignments` (`narrative_step_id` ASC);

-- -----------------------------------------------------
-- Table `Samples`
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS `Samples` (
  `sample_id` INTEGER PRIMARY KEY AUTOINCREMENT,
  `source_track_id` INTEGER NOT NULL, -- The track this sample is FROM
  `description` TEXT NULL,
  `start_time_ms` INTEGER NOT NULL,
  `end_time_ms` INTEGER NOT NULL,
  `sample_filepath` TEXT UNIQUE NULL, -- Relative path; Nullable if not yet extracted
  `loopable` BOOLEAN NOT NULL DEFAULT 0,
  `bpm` REAL NULL,
  `key` TEXT NULL,
  FOREIGN KEY (`source_track_id`) REFERENCES `Tracks` (`track_id`)
    ON DELETE CASCADE ON UPDATE NO ACTION
);
CREATE INDEX IF NOT EXISTS `Samples_source_track_id_idx` ON `Samples` (`source_track_id` ASC);

-- -----------------------------------------------------
-- Table `Audio_Analysis_Results`
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS `Audio_Analysis_Results` (
  `analysis_id` INTEGER PRIMARY KEY AUTOINCREMENT,
  `track_id` INTEGER NULL,
  `stem_id` INTEGER NULL,
  `sample_id` INTEGER NULL,
  `analyser_name` TEXT NULL,
  `bpm_detected` REAL NULL,
  `key_detected` TEXT NULL,
  `key_confidence` REAL NULL,
  `loudness_lufs_integrated` REAL NULL,
  `loudness_lufs_short_term_max` REAL NULL,
  `true_peak_dbfs` REAL NULL,
  `dynamic_range` REAL NULL,
  `beat_grid_json` TEXT NULL, -- JSON
  `segments_json` TEXT NULL, -- JSON
  `spectral_centroid_json` TEXT NULL, -- JSON
  `chroma_vector_json` TEXT NULL, -- JSON
  `advanced_features_json` TEXT NULL, -- JSON
  `analysis_timestamp` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (`track_id`) REFERENCES `Tracks` (`track_id`) ON DELETE CASCADE ON UPDATE NO ACTION,
  FOREIGN KEY (`stem_id`) REFERENCES `Stems` (`stem_id`) ON DELETE CASCADE ON UPDATE NO ACTION,
  FOREIGN KEY (`sample_id`) REFERENCES `Samples` (`sample_id`) ON DELETE CASCADE ON UPDATE NO ACTION,
  UNIQUE (`track_id`), -- A track has one primary analysis
  UNIQUE (`stem_id`),   -- A stem has one primary analysis
  UNIQUE (`sample_id`), -- A sample has one primary analysis
  CONSTRAINT `chk_analysis_target` CHECK (
    (`track_id` IS NOT NULL AND `stem_id` IS NULL AND `sample_id` IS NULL) OR
    (`track_id` IS NULL AND `stem_id` IS NOT NULL AND `sample_id` IS NULL) OR
    (`track_id` IS NULL AND `stem_id` IS NULL AND `sample_id` IS NOT NULL)
  )
);
CREATE INDEX IF NOT EXISTS `Audio_Analysis_Results_track_id_idx` ON `Audio_Analysis_Results` (`track_id` ASC);
CREATE INDEX IF NOT EXISTS `Audio_Analysis_Results_stem_id_idx` ON `Audio_Analysis_Results` (`stem_id` ASC);
CREATE INDEX IF NOT EXISTS `Audio_Analysis_Results_sample_id_idx` ON `Audio_Analysis_Results` (`sample_id` ASC);

-- -----------------------------------------------------
-- Table `Remix_Structure_Components`
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS `Remix_Structure_Components` (
  `component_id` INTEGER PRIMARY KEY AUTOINCREMENT,
  `narrative_step_id` INTEGER NOT NULL,
  `sequence_num` INTEGER NOT NULL, -- 1-3 per step
  `scene_num` INTEGER NOT NULL, -- 1-2 per sequence
  `phrase_num` INTEGER NOT NULL, -- 1-4 per scene
  `phrase_function` TEXT NOT NULL, -- e.g., "Hook 1 Male", "Bridge Female"
  `target_length_bars` INTEGER NULL DEFAULT 16,
  `target_length_ms` INTEGER NULL,
  `description` TEXT NULL,
  FOREIGN KEY (`narrative_step_id`) REFERENCES `Narrative_Steps` (`step_id`)
    ON DELETE CASCADE ON UPDATE NO ACTION,
  UNIQUE (`narrative_step_id`, `sequence_num`, `scene_num`, `phrase_num`)
);
CREATE INDEX IF NOT EXISTS `Remix_Structure_Components_narrative_step_id_idx` ON `Remix_Structure_Components` (`narrative_step_id` ASC);

-- -----------------------------------------------------
-- Table `Remix_Component_Assignments`
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS `Remix_Component_Assignments` (
  `assignment_id` INTEGER PRIMARY KEY AUTOINCREMENT,
  `component_id` INTEGER NOT NULL, -- FK to Remix_Structure_Components
  `element_order` INTEGER NOT NULL DEFAULT 0, -- For layering
  `track_id` INTEGER NULL, -- FK to Tracks (if snippet of full track)
  `sample_id` INTEGER NULL, -- FK to Samples (if pre-defined sample)
  `stem_id` INTEGER NULL, -- FK to Stems (if specific stem)
  `start_time_in_source_ms` INTEGER NULL, -- Start time in the source track/sample/stem
  `end_time_in_source_ms` INTEGER NULL,   -- End time in the source track/sample/stem
  `pitch_shift_semitones` INTEGER NULL DEFAULT 0,
  `time_stretch_ratio` REAL NULL DEFAULT 1.0,
  `midi_sequence_filepath` TEXT NULL, -- Relative path
  `instrument_notes` TEXT NULL, -- e.g., "Roland S-1, Patch A01"
  `fx_chain_description` TEXT NULL, -- e.g., "DS-1 (Drive 70%)"
  `notes` TEXT NULL,
  FOREIGN KEY (`component_id`) REFERENCES `Remix_Structure_Components` (`component_id`)
    ON DELETE CASCADE ON UPDATE NO ACTION,
  FOREIGN KEY (`track_id`) REFERENCES `Tracks` (`track_id`)
    ON DELETE SET NULL ON UPDATE NO ACTION,
  FOREIGN KEY (`sample_id`) REFERENCES `Samples` (`sample_id`)
    ON DELETE SET NULL ON UPDATE NO ACTION,
  FOREIGN KEY (`stem_id`) REFERENCES `Stems` (`stem_id`)
    ON DELETE SET NULL ON UPDATE NO ACTION,
  CONSTRAINT `chk_component_source` CHECK (
    `track_id` IS NOT NULL OR
    `sample_id` IS NOT NULL OR
    `stem_id` IS NOT NULL OR
    `midi_sequence_filepath` IS NOT NULL
  )
);
CREATE INDEX IF NOT EXISTS `Remix_Component_Assignments_component_id_idx` ON `Remix_Component_Assignments` (`component_id` ASC);
CREATE INDEX IF NOT EXISTS `Remix_Component_Assignments_track_id_idx` ON `Remix_Component_Assignments` (`track_id` ASC);
CREATE INDEX IF NOT EXISTS `Remix_Component_Assignments_sample_id_idx` ON `Remix_Component_Assignments` (`sample_id` ASC);
CREATE INDEX IF NOT EXISTS `Remix_Component_Assignments_stem_id_idx` ON `Remix_Component_Assignments` (`stem_id` ASC);

-- -----------------------------------------------------
-- Initial Data
-- -----------------------------------------------------

-- Stem_Types
INSERT INTO `Stem_Types` (name) VALUES
    ('Vocals'),
    ('Drums'),
    ('Bass'),
    ('Guitar'),
    ('Piano'),
    ('Other Instruments')
ON CONFLICT(name) DO NOTHING;

-- Narrative_Frameworks
INSERT INTO `Narrative_Frameworks` (name, description) VALUES
    ('Dan Harmon''s Story Circle', 'An 8-step narrative structure focusing on character transformation.')
ON CONFLICT(name) DO NOTHING;

-- Narrative_Steps for Dan Harmon's Story Circle
-- Assuming the framework_id for "Dan Harmon's Story Circle" will be 1 if it's the first one inserted.
-- You might need to SELECT the framework_id if running this script multiple times or out of order.
INSERT INTO `Narrative_Steps` (framework_id, name, step_order, description) VALUES
    ( (SELECT framework_id from Narrative_Frameworks WHERE name = 'Dan Harmon''s Story Circle'), 'YOU', 1, 'Protagonist in their zone of comfort.'),
    ( (SELECT framework_id from Narrative_Frameworks WHERE name = 'Dan Harmon''s Story Circle'), 'NEED', 2, 'But they want something.'),
    ( (SELECT framework_id from Narrative_Frameworks WHERE name = 'Dan Harmon''s Story Circle'), 'GO', 3, 'They enter an unfamiliar situation.'),
    ( (SELECT framework_id from Narrative_Frameworks WHERE name = 'Dan Harmon''s Story Circle'), 'SEARCH', 4, 'Adapt to it.'),
    ( (SELECT framework_id from Narrative_Frameworks WHERE name = 'Dan Harmon''s Story Circle'), 'FIND', 5, 'Get what they wanted.'),
    ( (SELECT framework_id from Narrative_Frameworks WHERE name = 'Dan Harmon''s Story Circle'), 'TAKE', 6, 'Pay a heavy price for it.'),
    ( (SELECT framework_id from Narrative_Frameworks WHERE name = 'Dan Harmon''s Story Circle'), 'RETURN', 7, 'Then return to their familiar situation.'),
    ( (SELECT framework_id from Narrative_Frameworks WHERE name = 'Dan Harmon''s Story Circle'), 'CHANGE', 8, 'Having changed.')
ON CONFLICT(framework_id, name) DO NOTHING;
