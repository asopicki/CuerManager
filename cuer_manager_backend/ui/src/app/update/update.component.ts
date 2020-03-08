import { Component, OnInit } from '@angular/core';

import { UpdateService } from './update.service'; 
import { map, catchError } from 'rxjs/operators';
import { Router } from '@angular/router';

@Component({
  selector: 'app-update',
  templateUrl: './update.component.html',
  styleUrls: ['./update.component.scss']
})
export class UpdateComponent implements OnInit {

  constructor(private service: UpdateService, private router: Router) { }

  ngOnInit() {
  }

  proceed(event: Event) {
    event.preventDefault();
    this.service.update().pipe(
      map(result => {
        this.router.navigate(['/home']);
      })).subscribe();
  }
}
