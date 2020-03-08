import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable } from 'rxjs';
import { map } from 'rxjs/operators';

@Injectable({
  providedIn: 'root'
})
export class UpdateService {

  constructor(private http: HttpClient) { }

  check(): Observable<boolean> {
    return this.http.get<boolean>('/v2/migrations/check');
  }

  update(): Observable<boolean> {
    console.log('update');
    return this.http.post<void>('/v2/migrations/run', "").pipe(
      map(() => true)
    );
  }
}
