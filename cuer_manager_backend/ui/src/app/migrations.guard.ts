import { Injectable } from '@angular/core';
import { CanActivate, ActivatedRouteSnapshot, RouterStateSnapshot, Router, UrlTree } from '@angular/router';

import { Observable, of } from 'rxjs';
import { map } from 'rxjs/operators';
import { UpdateService } from './update/update.service';

@Injectable({
  providedIn: 'root',
})
export class MigrationsGuard implements CanActivate {

    pendingMigrations: String

    constructor(private service: UpdateService, private router: Router) {

    }

  canActivate(
    next: ActivatedRouteSnapshot,
    state: RouterStateSnapshot): Observable<boolean | UrlTree> {
        if (this.pendingMigrations) {
            return of(true);
        }

        return this.service.check().pipe(
            map(pendingMigrations => {
                if (pendingMigrations) {
                    return this.router.parseUrl('/update');
                }

                this.pendingMigrations = "";

                return true;
            })
        );
    }
}