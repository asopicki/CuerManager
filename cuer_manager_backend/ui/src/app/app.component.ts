import { Component } from '@angular/core';

import { CuecardService } from './cuecard//cuecard.service';
import { MessageService } from './message.service';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss']
})
export class AppComponent {
  title = 'cuer-manager-ui';

  constructor(private service: CuecardService, private messageService: MessageService) {

  }

  refresh() {
    this.service.refresh().subscribe(
      () => { 
        this.service.announceLibraryRefreshed();
        this.messageService.info("Library refresh done.") 
      },
      (_) => this.messageService.error("Library update failed.")
    );
  }
}
