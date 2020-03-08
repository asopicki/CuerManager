import { Component, OnInit, OnDestroy, ChangeDetectionStrategy } from '@angular/core';
import { FormControl, FormGroup} from '@angular/forms';
import { debounceTime, distinctUntilChanged } from 'rxjs/operators';
import { MatDialog } from '@angular/material/dialog';

import { SearchService } from '../search/search.service';
import { Cuecard } from '../events/cuecard';
import { MetaDataEditorData, MetadataEditorComponent } from './metadata-editor/metadata-editor.component';
import { rhythms, phases } from '../shared/rhythms';
import { MetaData, CuecardService } from '../cuecard/cuecard.service';
import { MessageService } from '../message.service';
import { FileConversionService } from './file-conversion.service';
import { Subscription } from 'rxjs';

@Component({
  selector: 'app-library',
  templateUrl: './library.component.html',
  styleUrls: ['./library.component.scss'],
  changeDetection: ChangeDetectionStrategy.Default
})
export class LibraryComponent implements OnInit, OnDestroy {

  cuecards: Cuecard[]
  phases: string[]
  rhythms: string[]
  searchForm: FormGroup

  refreshSubscription: Subscription;

  constructor(
    private searchService: SearchService, 
    private dialog: MatDialog,
    private cuecardService: CuecardService,
    private messageService: MessageService,
    private fileService: FileConversionService) { 
    this.cuecards = [];

    this.phases = phases;
    this.rhythms = rhythms;

    let titleControl = new FormControl(null);
    titleControl.valueChanges.pipe(
      debounceTime(200), distinctUntilChanged()
    ).subscribe((title: string) => {
      if (title && title.length > 2) {
        this.searchTitle(title);
      }
    });

    let rhythmControl = new FormControl(null);

    this.searchForm = new FormGroup({
      title: titleControl,
      rhythm: rhythmControl
    });

    this.getCuecards();

    fileService.$libraryUpdated.subscribe(update => {
      this.resetFilter();
    })

    this.refreshSubscription = this.cuecardService.$libraryRefreshed.subscribe(updated => {
      this.resetFilter();
    });
  }

  ngOnInit() {

  }

  ngOnDestroy() {
    this.refreshSubscription.unsubscribe();
  }

  searchTitle(title: string) {
    let collator = Intl.Collator();
    this.searchService.byTitle(title).subscribe(cuecards => {
      this.updateCuecards(cuecards, (a: Cuecard, b: Cuecard) => {
        return collator.compare(a.title.toString(), b.title.toString());
      });
    });
  }

  searchPhase(phase: string) {
    let collator = Intl.Collator();
    this.searchService.byPhase(phase).subscribe(cuecards => {
      this.updateCuecards(cuecards, (a: Cuecard, b: Cuecard) => {
        if (collator.compare(a.phase.toString(), b.phase.toString()) === 0) {
          return collator.compare(a.title.toString(), b.title.toString());
        } else {
          collator.compare(a.phase.toString(), b.phase.toString());
        }
      });
    })
  }

  rhythmChanged(event) {
    if (event.value) {
      let collator = Intl.Collator();
      this.searchService.byRhythm(event.value).subscribe(cuecards => {
        this.updateCuecards(cuecards, (a: Cuecard, b: Cuecard) => {
          if (collator.compare(a.rhythm.toString(), b.rhythm.toString()) === 0) {
            return collator.compare(a.title.toString(), b.title.toString());
          } else {
            collator.compare(a.rhythm.toString(), b.rhythm.toString());
          }
        });
      });
    }
  }

  updateCuecards(cuecards: Cuecard[], sortFunc) {
    this.cuecards = cuecards.sort(sortFunc);
  }

  open(cuecard: Cuecard) {
    window.open(cuecard.getLink().toString(), cuecard.asTarget().toString());
  }

  onedit(cuecard: Cuecard) {
    let data = new MetaDataEditorData(cuecard);

    const dialogRef = this.dialog.open(MetadataEditorComponent, {
      data: data,
      height: '70%',
      width: '60%'
    });

    dialogRef.afterClosed().subscribe(result => {
      if (!result) {
        return;
      }

      console.log("dialog closed", "Result: ", result);
      let data = new MetaData();

      data.choreographer = result.choreographer;
      data.phase = result.phase;
      data.difficulty = result.difficulty || null;
      data.rhythm = result.rhythm;
      data.plusfigures = result.plusfigures || null;
      data.steplevel = result.steplevel || null;
      data.music = result.music || null;
      data.music_file = result.music_file || null;

      this.cuecardService.saveMetaData(cuecard, data).subscribe(() => {
        let updated_cuecard = this.cuecards.find(card => card.uuid === cuecard.uuid);

        updated_cuecard.choreographer = result.choreographer;
        updated_cuecard.phase = result.phase;
        updated_cuecard.difficulty = result.difficulty;
        updated_cuecard.rhythm = result.rhythm;
        updated_cuecard.steplevel = result.steplevel;
        updated_cuecard.meta['steplevel'] = result.steplevel;
        updated_cuecard.music_file = result.music_file;
        updated_cuecard.meta['music'] = result.music;
        updated_cuecard.meta['plusfigures'] = result.plusfigures;
        this.messageService.info("Changes have been saved.");
      }, () => {
        this.messageService.error("Error saving meta data!");
      });

      
    })
  }

  resetFilter() {
    this.searchForm.reset();
    this.getCuecards();
  }

  getCuecards() {
    this.searchService.getAll().subscribe(result => {
      this.updateCuecards(result, (a, b) => a.title.localeCompare(b.title.toString()));
    })
  }

  selectFile() {
    let file_input = <HTMLInputElement>document.getElementById('odtfile');

    file_input.dispatchEvent(new MouseEvent('click'));
  }

  convertFile(event: Event) {
    event.preventDefault();
    event.stopPropagation();

    let target = <HTMLInputElement>event.target;
    
    if (target.files.length > 0) {
      let reader = new FileReader();
      (<any>target.files[0]).arrayBuffer().then((buffer) => this.fileService.convertOdtFile(buffer, target.files[0].name));
    }
  }
}
