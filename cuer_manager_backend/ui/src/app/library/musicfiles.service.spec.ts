import { TestBed } from '@angular/core/testing';

import { MusicfilesService } from './musicfiles.service';

describe('MusicfilesService', () => {
  beforeEach(() => TestBed.configureTestingModule({}));

  it('should be created', () => {
    const service: MusicfilesService = TestBed.get(MusicfilesService);
    expect(service).toBeTruthy();
  });
});
