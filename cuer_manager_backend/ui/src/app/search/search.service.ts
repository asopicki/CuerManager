import { Injectable } from '@angular/core';
import { HttpClient, HttpHeaders } from '@angular/common/http';
import { Observable, of } from 'rxjs';
import { map } from 'rxjs/operators';

import { Cuecard } from '../events/cuecard';
import { MessageService } from '../message.service';
import { Message, MessageType } from '../message';

const urls = {
  "search": "v2/search",
  "all": "v2/cuecards"
}

const httpOptions = {
  headers: new HttpHeaders({ 'Content-Type': 'application/json' })
}

@Injectable({
  providedIn: 'root'
})
export class SearchService {

  constructor(private http: HttpClient, private messageService: MessageService) { }

  private log(message: string) {
    let msg = new Message(MessageType.ErrorMessage, `EventService: ${message}`);

    this.messageService.add(msg);
  }

  getAll(): Observable<Cuecard[]> {
    return this.http.get<Cuecard[]>(urls['all'])
      .pipe(map(cuecards => {
        const result = cuecards.map(data => new Cuecard(data));
        return result;
    }));
  }

  byTitle(title: string): Observable<Cuecard[]> {
    return this.http.get<Cuecard[]>(urls['search']+'/'+title)
      .pipe(map(cuecards => {
        const result = cuecards.map(data => new Cuecard(data));
        return result;
    }));
  }

  byRhythm(rhythm: string): Observable<Cuecard[]> {
    let query = 'rhythm:'+rhythm;

    return this.http.get<Cuecard[]>(urls['search']+'/'+query)
      .pipe(map(cuecards => {
        const result = cuecards.map(data => new Cuecard(data));
        return result;
    }));
  }

  byPhase(phase: string): Observable<Cuecard[]> {

    let query = 'phase:'+phase;

    return this.http.get<Cuecard[]>(urls['search']+'/'+query)
      .pipe(map(cuecards => {
        const result = cuecards.map(data => new Cuecard(data));
        return result;
    }));
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
