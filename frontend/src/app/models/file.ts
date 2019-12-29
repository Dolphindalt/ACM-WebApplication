export class File {
    file_id: number;
    uploader: number;
    audience: number;
    file_name: String;
    description: String;
};

export class EventFile {
    dummy_id: number;
    file_id: number;
    event_id: number;
    additional_info: String;
};