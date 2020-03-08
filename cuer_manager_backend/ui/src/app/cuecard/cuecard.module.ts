import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { CuecardComponent } from './cuecard.component';
import { CuecardRoutingModule } from './cuecard-routing.module';
import { PlayerComponent } from './player/player.component';



@NgModule({
  declarations: [CuecardComponent, PlayerComponent],
  imports: [
    CommonModule,
    CuecardRoutingModule
  ],
  exports: [
    CuecardComponent,
  ]
})
export class CuecardModule { }
