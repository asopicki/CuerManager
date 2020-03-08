import { Component, OnInit } from '@angular/core';
import { FormControl, FormGroup, Validators} from '@angular/forms';
import { MatDialogRef } from '@angular/material/dialog';
import { debounceTime, distinctUntilChanged } from 'rxjs/operators';

import { SearchService } from '../search.service';
import { Cuecard } from '../../events/cuecard';
import { rhythms, phases } from '../../shared/rhythms';

@Component({
  selector: 'app-search-dialog',
  templateUrl: './search-dialog.component.html',
  styleUrls: ['./search-dialog.component.scss']
})
export class SearchDialogComponent implements OnInit {

  searchForm: FormGroup
  cuecard: FormControl
  phases: string[]
  rhythms: string[]
  cuecards: Cuecard[]

  constructor(public dialogRef: MatDialogRef<SearchDialogComponent>, private searchService: SearchService) { 
    this.phases = phases;
    this.rhythms = rhythms;
    this.cuecards = []

    let titleControl = new FormControl(null);
    titleControl.valueChanges.pipe(
      debounceTime(200), distinctUntilChanged()
    ).subscribe((title: string) => {
      if (title && title.length > 2) {
        this.searchTitle(title);
      }
    });

    this.cuecard = new FormControl(null, [Validators.required])

    this.searchForm = new FormGroup({
      cuecard: this.cuecard,
      title: titleControl
    });
  }

  ngOnInit() {
  }

  onNoClick(): void {
    this.dialogRef.close();
  }

  searchTitle(title: string) {
    this.searchService.byTitle(title).subscribe(cuecards => {
      this.updateCuecards(cuecards)
    });
  }

  rhythmChanged(event) {
    if (event.value) {
      this.searchService.byRhythm(event.value).subscribe(cuecards => {
        this.updateCuecards(cuecards)
      });
    }
  }

  searchPhase(phase: string) {
    this.searchService.byPhase(phase).subscribe(cuecards => {
      this.updateCuecards(cuecards);
    })
  }

  updateCuecards(cuecards: Cuecard[]) {
    this.cuecards = cuecards.sort((a, b) => a.title.localeCompare(b.title.toString()));
    if (cuecards) {
      this.cuecard.setValue(this.cuecards[0])
    }
  }
}
