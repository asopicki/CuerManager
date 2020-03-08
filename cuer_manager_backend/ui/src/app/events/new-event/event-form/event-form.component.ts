import { Component, OnInit } from '@angular/core';
import { FormBuilder, FormGroup, Validators } from '@angular/forms';
import { EventService } from '../../event.service';

@Component({
  selector: 'app-event-form',
  templateUrl: './event-form.component.html',
  styleUrls: ['./event-form.component.scss']
})
export class EventFormComponent implements OnInit {

  newEventForm: FormGroup

  constructor(private eventService: EventService, private formBuilder: FormBuilder) { 
    this.newEventForm = formBuilder.group({
      name: ['', [Validators.required, Validators.minLength(3)] ],

    })
  }

  ngOnInit() {
  }


  get name() { return this.newEventForm.get('name'); }

  onSubmit() {
    let data = this.newEventForm.value;

    //this.eventService.createEvent(data.name, data.startDate).subscribe(() => console.log('Event created!'));
  }
}
