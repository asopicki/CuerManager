import { Component, OnInit } from '@angular/core';
import { MatDialogRef } from '@angular/material/dialog';
import { FormControl, FormGroup, Validators, ValidationErrors} from '@angular/forms';

import { DateTime } from 'luxon';

@Component({
  selector: 'app-new-event',
  templateUrl: './new-event.component.html',
  styleUrls: ['./new-event.component.scss']
})
export class NewEventComponent implements OnInit {

  newEventForm: FormGroup

  constructor(
    public dialogRef: MatDialogRef<NewEventComponent>) {
      this.newEventForm = new FormGroup ({
        name: new FormControl('', [Validators.required, Validators.minLength(3), Validators.maxLength(255)]),
        startDate: new FormControl(null, [Validators.required]),
        startTime: new FormControl('', [Validators.required]),
        endDate: new FormControl(null, [Validators.required]),
        endTime: new FormControl('', [Validators.required]),
      }, {
        validators: [this.isValidEndDate, this.isValidEndTime]
      });
    }

  ngOnInit() {
  }

  onNoClick(): void {
    this.dialogRef.close();
  }

  onDateChanged(group: FormGroup) {
    let startDate: Date|null = group.get('startDate').value;
    
    if (startDate) {
      group.get('endDate').setValue(startDate);
    }
  }

  get name(): FormControl {
    return <FormControl>this.newEventForm.get('name');
  }

  get startDate(): FormControl {
    return <FormControl>this.newEventForm.get('startDate');
  }

  get endDate(): FormControl {
    return <FormControl>this.newEventForm.get('endDate');
  }

  get startTime(): FormControl {
    return <FormControl>this.newEventForm.get('startTime');
  }

  get endTime(): FormControl {
    return <FormControl>this.newEventForm.get('endTime');
  }

  /*
   * validate that startDate is before endDate or same day
   */
  isValidEndDate(group: FormGroup): ValidationErrors | null  {
    let startDate: Date|null = group.get('startDate').value;
    let endDate:Date|null = group.get('endDate').value;

    if (startDate && endDate && startDate.getTime() <= endDate.getTime()) {
      return null;
    }

    return {'invalidEndDate': true}
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
