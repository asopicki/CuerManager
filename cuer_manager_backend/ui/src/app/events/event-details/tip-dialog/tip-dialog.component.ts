import { Component, OnInit, Inject } from '@angular/core';
import { FormControl, FormGroup, Validators, ValidationErrors} from '@angular/forms';
import { MatDialogRef, MAT_DIALOG_DATA } from '@angular/material/dialog';

import { DateTime } from 'luxon';

export enum TipDialogMode {
  Add,
  Update
}

export interface TipDialogData {
  name: string | null
  startTime: string
  endTime: string
  mode: TipDialogMode
}

@Component({
  selector: 'app-tip-dialog',
  templateUrl: './tip-dialog.component.html',
  styleUrls: ['./tip-dialog.component.scss']
})
export class TipDialogComponent implements OnInit {

  newTipForm: FormGroup
  mode: TipDialogMode

  constructor(public dialogRef: MatDialogRef<TipDialogComponent>, @Inject(MAT_DIALOG_DATA) public data: TipDialogData) {
    this.mode = data.mode;
    this.newTipForm = new FormGroup({
      name: new FormControl(data.name || '', [Validators.required, Validators.minLength(3), Validators.maxLength(255)]),
      startTime: new FormControl(data.startTime, [Validators.required]),
      endTime: new FormControl(data.endTime, [Validators.required])
    }, {
      validators: [this.isValidEndTime]
    })
  }

  ngOnInit() {
  }

  onNoClick(): void {
    this.dialogRef.close();
  }

  get title() {
    return this.mode == TipDialogMode.Add ? 'Add new tip' : 'Edit tip';
  }

  get buttonLabel() {
    return this.mode == TipDialogMode.Add ? 'Add' : 'Update';
  }

  get name() {
    return this.newTipForm.get('name');
  }

  get startTime() {
    return this.newTipForm.get('startTime');
  }

  get endTime() {
    return this.newTipForm.get('endTime');
  }

  /*
   * validate that startTime is before endTime
   */
  isValidEndTime(group: FormGroup): ValidationErrors | null  {
    let startTime:string = group.get('startTime').value;
    let endTime:string = group.get('endTime').value;

    if (startTime && endTime ) {
      let startDate = DateTime.fromISO(startTime);
      let endDate = DateTime.fromISO(endTime);

      if (startDate < endDate) {
        return null;
      }
    }

    return {'invalidEndTime': true}
  }
}
