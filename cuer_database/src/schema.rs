table! {
    cardindex_content (docid) {
        docid -> Nullable<Integer>,
        c0title -> Nullable<Binary>,
        c1choreographer -> Nullable<Binary>,
        c2meta -> Nullable<Binary>,
        c3content -> Nullable<Binary>,
    }
}

table! {
    cardindex_docsize (docid) {
        docid -> Nullable<Integer>,
        size -> Nullable<Binary>,
    }
}

table! {
    cardindex_segdir (level, idx) {
        level -> Nullable<Integer>,
        idx -> Nullable<Integer>,
        start_block -> Nullable<Integer>,
        leaves_end_block -> Nullable<Integer>,
        end_block -> Nullable<Integer>,
        root -> Nullable<Binary>,
    }
}

table! {
    cardindex_segments (blockid) {
        blockid -> Nullable<Integer>,
        block -> Nullable<Binary>,
    }
}

table! {
    cardindex_stat (id) {
        id -> Nullable<Integer>,
        value -> Nullable<Binary>,
    }
}

table! {
    cuecard_tags (id) {
        id -> Integer,
        cuecard_id -> Integer,
        tag_id -> Integer,
    }
}

table! {
    cuecards (id) {
        id -> Integer,
        uuid -> Text,
        phase -> Text,
        rhythm -> Text,
        title -> Text,
        steplevel -> Text,
        difficulty -> Text,
        choreographer -> Text,
        meta -> Text,
        content -> Text,
        karaoke_marks -> Text,
        music_file -> Text,
        file_path -> Text,
    }
}

table! {
    event_tags (id) {
        id -> Integer,
        event_id -> Integer,
        tag_id -> Integer,
    }
}

table! {
    events (id) {
        id -> Integer,
        uuid -> Text,
        date_start -> Text,
        date_end -> Text,
        name -> Text,
        schedule -> Nullable<Text>,
        date_created -> Text,
        date_modified -> Text,
    }
}

table! {
    playlist_cuecards (id) {
        id -> Integer,
        playlist_id -> Integer,
        cuecard_id -> Integer,
    }
}

table! {
    playlists (id) {
        id -> Integer,
        uuid -> Text,
        name -> Text,
    }
}

table! {
    programs (id) {
        id -> Integer,
        uuid -> Text,
        notes -> Nullable<Text>,
        event_id -> Integer,
        date_created -> Text,
        date_modified -> Text,
    }
}

table! {
    tags (id) {
        id -> Integer,
        tag -> Text,
    }
}

table! {
    tip_cuecards (id) {
        id -> Integer,
        tip_id -> Integer,
        cuecard_id -> Integer,
        sort_order -> Integer,
    }
}

table! {
    tips (id) {
        id -> Integer,
        uuid -> Text,
        name -> Text,
        program_id -> Integer,
        date_start -> Text,
        date_end -> Text,
    }
}

table! {
   cardindex(rowid) {
       rowid -> Integer,
       docid -> Integer,
       title -> Text,
       choreographer -> Text,
       meta -> Text,
       content -> Text,
   }
}

joinable!(playlist_cuecards -> playlists (playlist_id));
joinable!(playlist_cuecards -> cuecards (cuecard_id));
joinable!(cardindex -> cuecards(docid));
joinable!(programs -> events(event_id));
joinable!(tips -> programs(program_id));
joinable!(tip_cuecards -> tips(tip_id));
joinable!(tip_cuecards -> cuecards(cuecard_id));
joinable!(event_tags -> tags(tag_id));
joinable!(event_tags -> events(event_id));
joinable!(cuecard_tags -> tags(tag_id));
joinable!(cuecard_tags -> cuecards(cuecard_id));

allow_tables_to_appear_in_same_query!(
    cardindex_content,
    cardindex_docsize,
    cardindex_segdir,
    cardindex_segments,
    cardindex_stat,
    cuecard_tags,
    cuecards,
    event_tags,
    events,
    playlist_cuecards,
    playlists,
    programs,
    tags,
    tip_cuecards,
    tips,
);
