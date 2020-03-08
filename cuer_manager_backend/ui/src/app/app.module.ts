import { BrowserModule } from '@angular/platform-browser';
import { NgModule } from '@angular/core';
import { HttpClientModule }    from '@angular/common/http';
import {MatSnackBarModule, MatSnackBar} from '@angular/material/snack-bar';

import { AppRoutingModule } from './app-routing.module';
import { AppComponent } from './app.component';
import { EventsModule } from './events/events.module';
import { CuecardModule } from './cuecard/cuecard.module';
import { MessagesComponent } from './messages/messages.component';
import { HomeComponent } from './home/home.component';
import {BrowserAnimationsModule} from '@angular/platform-browser/animations';
import { UpdateComponent } from './update/update.component';
import { LibraryModule } from './library/library.module';


@NgModule({
  declarations: [
    AppComponent,
    MessagesComponent,
    HomeComponent,
    UpdateComponent,
  ],
  imports: [
    MatSnackBarModule,
    BrowserModule,
    AppRoutingModule,
    HttpClientModule,
    EventsModule,
    CuecardModule,
    BrowserAnimationsModule,
    LibraryModule
  ],
  providers: [MatSnackBar],
  bootstrap: [AppComponent],
  entryComponents: [MessagesComponent]
})
export class AppModule { }
