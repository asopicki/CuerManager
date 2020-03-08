import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';
import { HttpClient, HttpHeaders } from '@angular/common/http';

export enum FileType {
  File = "File",
  Directory = "Directory"
}

export class MusicFileEntry {
  file_name: String
  file_type: FileType
  parent_path: String
  separator: String
}

@Injectable({
  providedIn: 'root'
})
export class MusicfilesService {

  constructor(private client: HttpClient) { }

  getRoot(): Observable<MusicFileEntry[]> {
    const httpOptions = {
      headers: new HttpHeaders({ 'Content-Type': 'application/json' }),
    };

    let data = {
      filename: MusicfilesService._b64EncodeUnicode("".toString())
    }
    return this.client.post<MusicFileEntry[]>('/v2/music_files', data, httpOptions);
  }

  getSub(parent: String): Observable<MusicFileEntry[]> {
    const httpOptions = {
      headers: new HttpHeaders({ 'Content-Type': 'application/json' }),
    };

    let path = parent;

    if (parent.startsWith('/')) {
      path = parent.substring(1);
    }

    let data = {
      filename: MusicfilesService._b64EncodeUnicode(path.toString())
    }
    return this.client.post<MusicFileEntry[]>('/v2/music_files', data, httpOptions);
  }

  private static _b64EncodeUnicode(str: string): string {
    if (window
        && "btoa" in window
        && "encodeURIComponent" in window) {
        return btoa(encodeURIComponent(str).replace(/%([0-9A-F]{2})/g, (match, p1) => {
            return String.fromCharCode(("0x" + p1) as any);
        }));
    }
  }
}
