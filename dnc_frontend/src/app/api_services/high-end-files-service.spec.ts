import { TestBed } from '@angular/core/testing';

import { HighEndFilesService } from './high-end-files-service';

describe('HighEndFilesService', () => {
  let service: HighEndFilesService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(HighEndFilesService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
