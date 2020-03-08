import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { ReactiveFormsModule, FormsModule } from '@angular/forms';

import {ScrollingModule} from '@angular/cdk/scrolling';
import {MatListModule} from '@angular/material/list';
import {MatCardModule} from '@angular/material/card';
import {MatChipsModule} from '@angular/material/chips';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatInputModule } from '@angular/material/input';
import { MatSelectModule } from '@angular/material/select';
import { MatButtonModule } from '@angular/material/button';
import {MatAutocompleteModule} from '@angular/material/autocomplete';
import {MatBadgeModule} from '@angular/material/badge';
import {MatTooltipModule} from '@angular/material/tooltip';
import { MatDialogModule } from '@angular/material/dialog';
import {MatTreeModule} from '@angular/material/tree';
import {CdkTreeModule} from '@angular/cdk/tree';
import {MatProgressBarModule} from '@angular/material/progress-bar';
import {MatIconModule} from '@angular/material/icon';

import { LibraryComponent } from './library.component';
import { LibraryRoutingModule } from './library-routing.module';
import { CuecardCardComponent } from './cuecard-card/cuecard-card.component';
import { TagsEditorComponent } from './tags-editor/tags-editor.component';
import { MetadataEditorComponent } from './metadata-editor/metadata-editor.component';
import { FileSelectorComponent } from './metadata-editor/file-selector/file-selector.component';

const modules = [
  ReactiveFormsModule,
  FormsModule,
  ScrollingModule,
  MatFormFieldModule,
  MatButtonModule,
  MatInputModule,
  MatSelectModule,
  MatCardModule,
  MatListModule,
  MatChipsModule,
  MatAutocompleteModule,
  MatBadgeModule,
  MatTooltipModule,
  MatDialogModule,
  CdkTreeModule,
  MatTreeModule,
  MatProgressBarModule,
  MatIconModule
]

@NgModule({
  declarations: [LibraryComponent, CuecardCardComponent, TagsEditorComponent, MetadataEditorComponent, FileSelectorComponent],
  imports: [
    ...modules,
    LibraryRoutingModule,
    CommonModule
  ],
  entryComponents: [TagsEditorComponent, MetadataEditorComponent, FileSelectorComponent],
  exports: [LibraryComponent, CuecardCardComponent]
})
export class LibraryModule { }
