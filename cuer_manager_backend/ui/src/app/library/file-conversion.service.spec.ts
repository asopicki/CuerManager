import { TestBed } from '@angular/core/testing';

import { FileConversionService } from './file-conversion.service';

describe('FileConversionService', () => {
  beforeEach(() => TestBed.configureTestingModule({}));

  it('should be created', () => {
    const service: FileConversionService = TestBed.get(FileConversionService);
    expect(service).toBeTruthy();
  });
});
