import { Injectable } from '@angular/core';
import { HttpClient, HttpHeaders } from '@angular/common/http';
import { Observable } from 'rxjs';

import { MessageService  } from './message.service';

const httpOptions = {
  headers: new HttpHeaders({ 'Content-Type': 'application/json' })
}

class Settings {
  minutes_per_tip: number

  constructor(minutes: number) {
    this.minutes_per_tip = minutes;
  }

}

@Injectable({
  providedIn: 'root'
})
export class SettingsService {

  constructor(private http: HttpClient, private messageService: MessageService) { }

  getSettings(): Observable<Settings> {
    return this.http.get<Settings>('/v2/settings');
  }
}
