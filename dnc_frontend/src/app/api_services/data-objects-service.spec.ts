import { TestBed } from '@angular/core/testing';

import { DataObjectsService } from './data-objects-service';

describe('DataObjectsService', () => {
  let service: DataObjectsService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(DataObjectsService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
