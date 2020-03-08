import { Component, OnInit, Inject, ViewChild } from '@angular/core';
import { FormControl, FormGroup, Validators } from '@angular/forms';
import {COMMA, ENTER} from '@angular/cdk/keycodes';
import { MatDialogRef, MAT_DIALOG_DATA } from '@angular/material/dialog';
import {MatAutocompleteSelectedEvent, MatAutocomplete} from '@angular/material/autocomplete';
import {MatChipInputEvent} from '@angular/material/chips';
import {Observable} from 'rxjs';
import {map, startWith} from 'rxjs/operators';

import { Cuecard } from 'src/app/events/cuecard';
import { Tag } from 'src/app/tag';
import { CuecardService } from 'src/app/cuecard/cuecard.service';

export interface TagsDataInterface {
  cuecard: Cuecard
}

export class TagsEditorData implements TagsDataInterface {
  cuecard: Cuecard
  
  constructor(cuecard: Cuecard) {
    this.cuecard = cuecard;
  }
}

@Component({
  selector: 'app-tags-editor',
  templateUrl: './tags-editor.component.html',
  styleUrls: ['./tags-editor.component.scss']
})
export class TagsEditorComponent implements OnInit {

  newTagGroup: FormGroup
  tag: FormControl
  cueCard: Cuecard
  tags: Tag[]
  allTags: Tag[]
  filteredTags: Observable<string[]>
  isRemovable = true
  addOnBlur = false
  keyCodes: number[] = [COMMA, ENTER]
  @ViewChild('tagauto', {static: false}) matAutocomplete: MatAutocomplete;

  selectedOption: any

  constructor(
    public dialogRef: MatDialogRef<TagsEditorComponent>, 
    @Inject(MAT_DIALOG_DATA) public data: TagsDataInterface,
    private cuecardService: CuecardService) {
    this.tag = new FormControl('', [Validators.required, Validators.minLength(3), Validators.maxLength(255)]);
    this.newTagGroup = new FormGroup({
      tag: this.tag,
    });
    this.cueCard = data.cuecard;

    this.filteredTags= this.tag.valueChanges.pipe(
      startWith(null),
      map((tag: string | null) => tag && tag.length > 2 ? this._filter(tag) : []));
  }

  ngOnInit() {
    this.cuecardService.getTags(this.cueCard.uuid).subscribe(tags => this.tags = tags);

    this.cuecardService.getAllTags().subscribe(tags => this.allTags = tags);
  }

  removeTag(tag: Tag) {
    this.cuecardService.removeTag(tag, this.cueCard).subscribe(() => {
      const index = this.tags.indexOf(tag);

      if (index >= 0) {
        this.tags.splice(index, 1);
      }
    });
  }

  addTag(event: MatChipInputEvent) {
    if (this.tag.valid && !this.matAutocomplete.isOpen) {
      let tag = new Tag({tag: this.tag.value});
      
      this.cuecardService.addTag(tag, this.cueCard).subscribe(() => this._addToTags(tag));
    }
  }

  selected(event: MatAutocompleteSelectedEvent) {
    const tagValue = event.option.value;

    this.tag.setValue(tagValue);
  }

  _addToTags(tag: Tag) {
    this.tags.push(tag);

    this.tag.setValue(null);
  }

  onCloseClick(): void {
    this.dialogRef.close();
  }

  private _filter(value: string): string[] {
    const filterValue = value.toLowerCase();

    return this.allTags
      .filter(tag => tag.tag.toLowerCase().startsWith(filterValue))
      .map(tag => tag.tag.toString());
  }
}
