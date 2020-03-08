import { Component, OnInit } from '@angular/core';
import { Router, ActivatedRoute } from '@angular/router';
import { MatDialog } from '@angular/material/dialog';

import { Event } from './event';
import { EventService } from './event.service';
import { NewEventComponent } from './new-event/new-event.component';

import { DateTime } from 'luxon';

@Component({
  selector: 'app-events',
  templateUrl: './events.component.html',
  styleUrls: ['./events.component.scss']
})
export class EventsComponent implements OnInit {

  events: Event[];
  startDate: String;
  endDate: String;

  constructor(private route: ActivatedRoute, private eventService: EventService, private router: Router, private dialog: MatDialog) {
    route.paramMap.subscribe((params) => {
      let date_start = params.get('date_start');
      let date_end = params.get('date_end');

      if (date_start && date_end) {
        let start = DateTime.fromFormat(date_start, 'yyyy-MM-dd');
        let end = DateTime.fromFormat(date_end, 'yyyy-MM-dd');
        
        if (start && end) {
          this.startDate = start.toUTC().toISO();
          this.endDate = end.toUTC().toISO();
          console.log(this.startDate, this.endDate);
        }
      } else {
        this.startDate = DateTime.local().minus({months: 2}).toUTC().toISO(); 
        this.endDate = DateTime.local().plus({months: 5}).toUTC().toISO();
      }
    })
  }

  getEvents() {
    if (this.startDate && this.endDate) {
      return this.eventService.getEvents(this.startDate, this.endDate)
      .subscribe(events => this.events = events);  
    }
    return this.eventService.getEvents()
      .subscribe(events => this.events = events);
  }

  ngOnInit() {
    this.getEvents();
  }

  previousMonth(event: any) {
    event.preventDefault();
    event.stopPropagation();

    let date_start: DateTime;
    let date_end: DateTime;


    date_start = DateTime.fromISO(this.startDate.toString()).minus({months: 1});
    date_end = DateTime.fromISO(this.endDate.toString()).minus({months: 1});

    this.startDate = date_start.toISO();
    this.endDate = date_end.toISO();
    this.getEvents();
  }

  reset(event: any) {
    event.preventDefault();
    this.startDate = null;
    this.endDate = null;
    this.getEvents();
  }

  nextMonth(event: any) {
    event.preventDefault();
    event.stopPropagation();

    let date_start: DateTime;
    let date_end: DateTime;

    date_start = DateTime.fromISO(this.startDate.toString()).plus({months: 1});
    date_end = DateTime.fromISO(this.endDate.toString()).plus({months: 1});

    this.startDate = date_start.toISO();
    this.endDate = date_end.toISO();
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

  showDetails(event: Event) {
    this.router.navigate(['/events/details/' + event.uuid]);

  }
}
