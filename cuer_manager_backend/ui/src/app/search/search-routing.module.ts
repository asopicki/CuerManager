import { NgModule, CUSTOM_ELEMENTS_SCHEMA } from '@angular/core';
import { Routes, RouterModule } from '@angular/router';
import { FormsModule } from '@angular/forms';

import { SearchComponent } from './search/search.component';
import { MigrationsGuard } from '../migrations.guard';

const routes: Routes = [
  { path: 'search', component: SearchComponent, canActivate: [MigrationsGuard]},
];

let modules =  [
  FormsModule,
   
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
 
  schemas: [CUSTOM_ELEMENTS_SCHEMA]
})
export class SearchRoutingModule { }
