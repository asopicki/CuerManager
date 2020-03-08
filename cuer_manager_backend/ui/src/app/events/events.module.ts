import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { DragDropModule } from '@angular/cdk/drag-drop';
import {MatListModule} from '@angular/material/list';
import {MatCardModule} from '@angular/material/card';
import {MatBadgeModule} from '@angular/material/badge';
import {MatTooltipModule} from '@angular/material/tooltip';

import { EventsComponent } from './events.component';
import { EventDetailsComponent } from './event-details/event-details.component';
import { NewEventComponent } from './new-event/new-event.component';
import { EventRoutingModule } from './event-routing.module';
import { EventFormComponent } from './new-event/event-form/event-form.component';
import { ReactiveFormsModule } from '@angular/forms';
import { TipDialogComponent } from './event-details/tip-dialog/tip-dialog.component';
import { SearchModule } from '../search/search.module';
import { NotesEditorComponent } from './event-details/notes-editor/notes-editor.component';
import { IssueBadgeComponent } from './event-details/issue-badge/issue-badge.component';
import { BarChartComponent } from './event-details/bar-chart/bar-chart.component';
import { ChartsModule } from 'ng2-charts';
import { EventReportComponent } from './event-report/event-report.component';

@NgModule({
  declarations: [
    EventsComponent, 
    EventDetailsComponent, 
    NewEventComponent, 
    EventFormComponent, 
    TipDialogComponent, 
    NotesEditorComponent, 
    IssueBadgeComponent,
    BarChartComponent,
    EventReportComponent
  ],
  entryComponents: [TipDialogComponent, NotesEditorComponent],
  imports: [
    CommonModule,
    EventRoutingModule,
    ReactiveFormsModule,
    MatCardModule,
    MatListModule,
    MatBadgeModule,
    MatTooltipModule,
    ChartsModule,
    SearchModule,
    DragDropModule
  ],
  exports: [
    EventsComponent
  ],
})
export class EventsModule { }
