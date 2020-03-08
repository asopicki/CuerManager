import { NgModule, CUSTOM_ELEMENTS_SCHEMA } from '@angular/core';
import { Routes, RouterModule } from '@angular/router';

import { MatDialogModule } from '@angular/material/dialog';

import { CuecardComponent } from './cuecard.component';
import { MigrationsGuard } from '../migrations.guard';

const routes: Routes = [
    { path: 'cuecard/:uuid', component: CuecardComponent, canActivate: [MigrationsGuard]},
];

@NgModule({
    imports: [
      RouterModule.forRoot(routes)
    ],
    exports: [
      MatDialogModule,
      RouterModule
    ],
    schemas: [ CUSTOM_ELEMENTS_SCHEMA ]
  })
  export class CuecardRoutingModule { }