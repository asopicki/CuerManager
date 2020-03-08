import { Injectable } from '@angular/core';
import { HttpHeaders, HttpClient } from '@angular/common/http';
import { Subject } from 'rxjs';

import { MessageService } from '../message.service';

@Injectable({
  providedIn: 'root'
})
export class FileConversionService {

  private libraryUpdated = new Subject<boolean>();

  $libraryUpdated = this.libraryUpdated.asObservable();

  constructor(private client: HttpClient, private messageService: MessageService) { }

  convertOdtFile(buffer: ArrayBuffer, filename: String) {
    let httpOptions = {
      headers: new HttpHeaders({ 'Content-Type': 'application/octet-stream' })
        .append('X-Input-FileName', filename.toString()),
      responseType: "blob" as "text"
    }
    
    this.client.post("/v2/convert/odt", buffer, httpOptions).subscribe((blob => {
      if (blob && blob.length) {
        let a = document.createElement('a');
        a.href = window.URL.createObjectURL(blob);
        let name = filename.replace('.odt', '.md');
        a.download = name;
        a.dispatchEvent(new MouseEvent('click'));
      } else {
        this.messageService.info('File has been converted and added to the library');
        this.announceUpdate();
      }
    }));
  }

  announceUpdate() {
    this.libraryUpdated.next(true);
  }
}
