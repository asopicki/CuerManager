import { Injectable } from '@angular/core';
import { HttpClient, HttpHeaders, HttpParams } from '@angular/common/http';
import { Observable, of } from 'rxjs';
import { map, catchError } from 'rxjs/operators'; //tap

import { OptionalProgram, Program } from './program';
import { Event } from './event';
import { MessageService  } from '../message.service';
import { Message, MessageType } from '../message';

import { DateTime } from 'luxon';

const urls = {
  "program_event": "v2/event/program/",
  "program_update_notes": "v2/program/"
};

const httpOptions = {
  headers: new HttpHeaders({ 'Content-Type': 'application/json' })
}

@Injectable({
  providedIn: 'root'
})
export class ProgramService {

  constructor(private http: HttpClient, private messageService: MessageService) { }

  getProgram(event: Event): Observable<OptionalProgram> {
    if (event) {
      return this.http.get<OptionalProgram>(urls['program_event'] + event.getId())
        .pipe(
          map(programData => {
            return programData ? new Program(programData, event) : null;
          }),
          catchError(this.handleError<null>('getProgram'))
        )
    } else {
      return of(null);
    }
  }

  getNotes(event: Event): Observable<string> {
    return this.http.get("v2/event/" + event.getId() + "/program/notes", {responseType:'text'})
      .pipe(
        map(data => data),
        catchError(this.handleError<string>('getNotes', ''))
      )
      
  }

  updateNotes(program: Program, notes: String): Observable<String> {
    const data = {
      notes: notes,
      date_modified: DateTime.utc().toISO()
    }
    return this.http.post(urls['program_update_notes'] + program.id + '/notes', data, {responseType:'text'})
      .pipe(
        catchError(this.handleError<String>('updateNotes', notes))
      )
  }

  private log(message: string) {
    let msg = new Message(MessageType.ErrorMessage, `ProgramService: ${message}`)

    this.messageService.add(msg);
  }

  /**
   * Handle Http operation that failed.
   * Let the app continue.
   * @param operation - name of the operation that failed
   * @param result - optional value to return as the observable result
   */
  private handleError<T> (operation = 'operation', result?: T) {
    return (error: any): Observable<T> => {
    
      console.error(error); // log to console instead
    
      // TODO: better job of transforming error for user consumption
      this.log(`${operation} failed: ${error.message}`);
    
      // Let the app keep running by returning an empty result.
      return of(result as T);
    };
  }
}
