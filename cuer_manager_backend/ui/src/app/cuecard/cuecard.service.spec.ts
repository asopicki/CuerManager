import { TestBed } from '@angular/core/testing';

import { CuecardService } from './cuecard.service';

describe('CuecardService', () => {
  beforeEach(() => TestBed.configureTestingModule({}));

  it('should be created', () => {
    const service: CuecardService = TestBed.get(CuecardService);
    expect(service).toBeTruthy();
  });
});
