import { Component, OnInit } from '@angular/core';
import { MatDialog } from '@angular/material/dialog';

import { Event } from '../events/event';
import { EventService } from '../events/event.service'
import { NewEventComponent } from '../events/new-event/new-event.component';

import { DateTime } from 'luxon';

@Component({
  selector: 'app-home',
  templateUrl: './home.component.html',
  styleUrls: ['./home.component.scss']
})
export class HomeComponent implements OnInit {

  events: Event[];

  constructor(private eventService: EventService, private dialog: MatDialog) { }

  getEvents() {
    let date_start = DateTime.local().minus({months: 1}).toUTC().toISO(); 
    let date_end = DateTime.local().plus({months: 4}).toUTC().toISO();
    return this.eventService.getEvents(date_start, date_end).subscribe(events => this.events = events.slice(0,8));
  }

  ngOnInit() {
    this.getEvents();
  }

  addEvent(event: any) {
    event.preventDefault();
    const dialogRef = this.dialog.open(NewEventComponent);

    dialogRef.afterClosed().subscribe(result => {
      if (!result) {
        return;
      }

      let startDate = DateTime.fromJSDate(result.startDate);
      let startTime = DateTime.fromISO(result.startTime);
      let endDate = DateTime.fromJSDate(result.endDate);
      let endTime = DateTime.fromISO(result.endTime);
      
      startDate = startDate.set({
        hour: startTime.hour,
        minute: startTime.minute,
        second: startTime.second,
        millisecond: startTime.millisecond
      });
      
      endDate = endDate.set({
        hour: endTime.hour,
        minute: endTime.minute,
        second: endTime.second,
        millisecond: endTime.millisecond
      })
      console.debug('Start date:', startDate);
      console.debug('End date:', endDate);

      this.eventService.createEvent(result.name, startDate, endDate).subscribe(_ => {
        this.getEvents();
      });
    });
  }
}
