import { Component, OnInit, Inject } from '@angular/core';
import {MAT_SNACK_BAR_DATA, MatSnackBarContainer} from '@angular/material';
import { Message, MessageType } from '../message';

@Component({
  selector: 'app-messages',
  templateUrl: './messages.component.html',
  styleUrls: ['./messages.component.scss'],
  entryComponents: [MatSnackBarContainer]
})
export class MessagesComponent implements OnInit {

  message: Message
  iconClass: String

  constructor(@Inject(MAT_SNACK_BAR_DATA) public data: any) {
    this.message = <Message>data.message;

    switch(this.message.type) {
      case MessageType.ErrorMessage: {
        this.iconClass = "fas fa-exclamation-circle error";
        break;
      }
      case MessageType.WarningMessage: {
        this.iconClass = "fas fa-exclamation-triangle warning";
        break;
      }
      default: {
        this.iconClass = "fas fa-info-circle info";
        break;
      }
    }
  }

  ngOnInit() {
  }
}
