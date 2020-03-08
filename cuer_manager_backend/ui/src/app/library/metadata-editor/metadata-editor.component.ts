import { Component, OnInit, Inject } from '@angular/core';
import { MatDialogRef, MAT_DIALOG_DATA, MatDialog } from '@angular/material/dialog';
import { FormControl, FormGroup, Validators } from '@angular/forms';

import { Cuecard } from 'src/app/events/cuecard';
import { rhythms, phases } from '../../shared/rhythms';
import { FileSelectorComponent } from './file-selector/file-selector.component';

export interface MetaDataEditorInterface {
  cuecard: Cuecard
}

export class MetaDataEditorData implements MetaDataEditorInterface {
  cuecard: Cuecard
  
  constructor(cuecard: Cuecard) {
    this.cuecard = cuecard;
  }
}

@Component({
  selector: 'app-metadata-editor',
  templateUrl: './metadata-editor.component.html',
  styleUrls: ['./metadata-editor.component.scss']
})
export class MetadataEditorComponent implements OnInit {

  metaDataGroup: FormGroup
  cueCard: Cuecard
  rhythms = rhythms;
  phases = phases;

  constructor(
    public dialogRef: MatDialogRef<MetadataEditorComponent>, 
    private dialog: MatDialog,
    @Inject(MAT_DIALOG_DATA) public data: MetaDataEditorInterface) {
      
  }

  ngOnInit() {
    this.cueCard = this.data.cuecard;
      
    this.metaDataGroup = new FormGroup({
      choreographer: new FormControl(this.cueCard.choreographer, [Validators.required, Validators.minLength(3), Validators.maxLength(255)]),
      phase: new FormControl(this.cueCard.phase, [Validators.required]),
      difficulty: new FormControl(this.cueCard.meta['difficulty'], [Validators.required, Validators.minLength(3), Validators.maxLength(255)]),
      rhythm: new FormControl(this.cueCard.rhythm, [Validators.required]),
      plusfigures: new FormControl(this.cueCard.meta['plusfigures'], [Validators.maxLength(255)]),
      steplevel: new FormControl(this.cueCard.meta['steplevel'], [Validators.maxLength(10)]),
      music: new FormControl(this.cueCard.meta['music'], [Validators.maxLength(255)]),
      music_file: new FormControl(this.cueCard.music_file, [Validators.maxLength(255)]),
    });
  }

  onCancelClick(): void {
    this.dialogRef.close();
  }

  selectMusicFile(): void {
    const dialogRef = this.dialog.open(FileSelectorComponent, {
      height: "80%",
      width: "90%"
    });

    dialogRef.afterClosed().subscribe(result => {
      if (!result) {
        return;
      }

      this.metaDataGroup.get('music_file').setValue(result);
      console.log(result);
    });
  }
}
