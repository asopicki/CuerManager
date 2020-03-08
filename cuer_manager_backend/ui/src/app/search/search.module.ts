import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { ReactiveFormsModule, FormsModule } from '@angular/forms';
import { SearchDialogComponent } from './search-dialog/search-dialog.component';
import { SearchRoutingModule } from './search-routing.module';

import { MatButtonModule } from '@angular/material/button';
import { MatDialogModule } from '@angular/material/dialog';
import { MatDividerModule } from '@angular/material/divider';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatInputModule } from '@angular/material/input';
import { MatSelectModule } from '@angular/material/select';
import { SearchComponent } from './search/search.component';
import { MatListModule } from '@angular/material/list';
import { LibraryModule } from '../library/library.module';

let modules =  [
  FormsModule,
  MatFormFieldModule,
  MatInputModule,
  MatDialogModule,
  MatButtonModule,
  MatSelectModule,
  MatDividerModule,
  MatListModule,
  LibraryModule
]

@NgModule({
  declarations: [SearchDialogComponent, SearchComponent],
  imports: [
    ...modules,
    CommonModule,
    ReactiveFormsModule,
    SearchRoutingModule
  ],
  exports: [
    ...modules,
    SearchComponent
  ],
  entryComponents: [ SearchDialogComponent ]
})
export class SearchModule { }
