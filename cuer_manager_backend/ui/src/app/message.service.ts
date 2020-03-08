import { Injectable } from '@angular/core';
import {MatSnackBar} from '@angular/material/snack-bar';
import { Message, MessageType } from './message';
import { MessagesComponent } from './messages/messages.component';

@Injectable({
  providedIn: 'root'
})
export class MessageService {
  messages: Message[] = [];
  duration = 5;
  showingMessage = false;

  constructor(private snackBar: MatSnackBar) { }

  info(msg: String) {
    this.add(new Message(MessageType.InfoMessage, msg))
  }

  warn(msg: String) {
    this.add(new Message(MessageType.WarningMessage, msg))
  }

  error(msg: String) {
    this.add(new Message(MessageType.ErrorMessage, msg))
  }
 
  add(message: Message) {
    this.messages.push(message);

    if (!this.showingMessage) {
      this.showingMessage = true;
      setTimeout(() => this._showMessage(), 0);
    }
  }
 
  _showMessage() {
    if (this.messages.length) {
      let message = this.messages.shift();

      this.snackBar.openFromComponent(MessagesComponent, {
        duration: this.duration * 1000,
        data: {
          message: message
        }
      });

      if (this.messages.length) {
        setTimeout(() => this._showMessage(), this.duration * 1000 + 1000);
      } else {
        this.showingMessage = false;
      }
    }
  }
  
}
