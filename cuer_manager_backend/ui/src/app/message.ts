
export enum MessageType {
    InfoMessage,
    WarningMessage,
    ErrorMessage
};

export class Message {
    type: MessageType
    title: String
    message: String

    constructor(type: MessageType, message: String) {
        this.type = type;
        this.message = message;

        switch(type) {
            case MessageType.ErrorMessage: {
                this.title = 'ERROR';
                break;
            }
            case MessageType.WarningMessage: {
                this.title = 'WARNING';
                break;
            }
            default: {
                this.title = 'Info';
                break;
            }
        }
    }
}