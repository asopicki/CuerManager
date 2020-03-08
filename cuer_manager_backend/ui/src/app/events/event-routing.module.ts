import { NgModule, CUSTOM_ELEMENTS_SCHEMA, Injectable } from '@angular/core';
import { Routes, RouterModule } from '@angular/router';
import { FormsModule } from '@angular/forms';

import { MatButtonModule } from '@angular/material/button';
import { MatNativeDateModule, NativeDateAdapter, DateAdapter } from '@angular/material/core';
import { MatDatepickerModule } from '@angular/material/datepicker';
import { MatDialogModule } from '@angular/material/dialog';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatInputModule } from '@angular/material/input';
import {NgxMaterialTimepickerModule} from 'ngx-material-timepicker';

import { EventDetailsComponent } from './event-details/event-details.component';
import { NewEventComponent } from './new-event/new-event.component';
import { EventsComponent } from './events.component';
import { MigrationsGuard } from '../migrations.guard';
import { EventReportComponent } from './event-report/event-report.component';

@Injectable()
export class CustomDateAdapter extends NativeDateAdapter {
  getFirstDayOfWeek(): number {
    return 1;
   }
}

const routes: Routes = [
  { path: 'events', component: EventsComponent, canActivate: [MigrationsGuard]},
  { path: 'events/new', component: NewEventComponent, canActivate: [MigrationsGuard] },
  { 
    path: 'events/details/:uuid', 
    component: EventDetailsComponent,
    canActivate: [MigrationsGuard]
  },
  {
    path: 'events/report/:uuid',
    component: EventReportComponent,
    canActivate: [MigrationsGuard]
  },
  { path: 'events/:date_start/:date_end', component: EventsComponent, canActivate: [MigrationsGuard]},
];

let modules =  [
  FormsModule,
    MatFormFieldModule,
    MatInputModule,
    MatDialogModule,
    MatButtonModule,
    MatDatepickerModule,
    MatNativeDateModule,
    NgxMaterialTimepickerModule
]

@NgModule({
  imports: [
    ...modules,
    RouterModule.forRoot(routes)
  ],
  exports: [
    ...modules,
    RouterModule
  ],
  providers: [
    { provide: DateAdapter, useClass: CustomDateAdapter },
  ],
  schemas: [CUSTOM_ELEMENTS_SCHEMA]
})
export class EventRoutingModule { }
