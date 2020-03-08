import { NgModule } from '@angular/core';
import { Routes, RouterModule } from '@angular/router';


import { HomeComponent } from './home/home.component';
import { MigrationsGuard } from './migrations.guard';
import { UpdateComponent } from './update/update.component';

const routes: Routes = [
  { path: 'home', component: HomeComponent, canActivate: [MigrationsGuard] },
  { path: '', redirectTo: 'home', pathMatch: 'full' },
  { path: 'update', component: UpdateComponent }
];

@NgModule({
  imports: [
    RouterModule.forRoot(routes)
  ],
  exports: [RouterModule]
})
export class AppRoutingModule { }
