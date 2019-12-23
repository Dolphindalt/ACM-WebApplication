import { TestBed } from '@angular/core/testing';

import { BaseurlService } from './baseurl.service';

describe('BaseurlService', () => {
  beforeEach(() => TestBed.configureTestingModule({}));

  it('should be created', () => {
    const service: BaseurlService = TestBed.get(BaseurlService);
    expect(service).toBeTruthy();
  });
});
