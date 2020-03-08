import { Component, OnInit } from '@angular/core';

import { SearchService } from '../search.service';
import { Cuecard } from '../../events/cuecard';
import { FormControl, FormGroup} from '@angular/forms';
import { debounceTime, distinctUntilChanged } from 'rxjs/operators';
import { rhythms, phases } from '../../shared/rhythms';

const urls = {
  'cuecard_data': '/cuecard/'
};

@Component({
  selector: 'app-search',
  templateUrl: './search.component.html',
  styleUrls: ['./search.component.scss']
})
export class SearchComponent implements OnInit {

  phases: string[]
  rhythms: string[]
  cuecards: Cuecard[]
  searchForm: FormGroup
  
  constructor(private searchService: SearchService) {
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

    let rhythmControl = new FormControl(null);

    this.searchForm = new FormGroup({
      title: titleControl,
      rhythm: rhythmControl
    });
  }

  ngOnInit() {
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

  plusFigures(cuecard: Cuecard): String {
    return cuecard.meta['plusfigures'];
  }

  cuecardLink(cuecard: Cuecard): String {
    return cuecard.getLink();
  }

  open(cuecard: Cuecard) {
    window.open(this.cuecardLink(cuecard).toString(), "_blank");
  }
}
