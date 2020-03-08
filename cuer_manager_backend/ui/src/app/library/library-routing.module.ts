import { NgModule } from '@angular/core';
import { Routes, RouterModule } from '@angular/router';

import { LibraryComponent } from './library.component';
import { MigrationsGuard } from '../migrations.guard';

const routes: Routes = [
  { path: 'library', component: LibraryComponent, canActivate: [MigrationsGuard]},
];


@NgModule({
  imports: [
    RouterModule.forRoot(routes)
  ],
  exports: [
    RouterModule
  ],
 
})
export class LibraryRoutingModule { }
