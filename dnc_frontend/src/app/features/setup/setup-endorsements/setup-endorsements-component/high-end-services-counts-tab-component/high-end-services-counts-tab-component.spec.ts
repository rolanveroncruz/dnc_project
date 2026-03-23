import { ComponentFixture, TestBed } from '@angular/core/testing';

import { HighEndServicesCountsTabComponent } from './high-end-services-counts-tab-component';

describe('HighEndServicesCountsTabComponent', () => {
  let component: HighEndServicesCountsTabComponent;
  let fixture: ComponentFixture<HighEndServicesCountsTabComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [HighEndServicesCountsTabComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(HighEndServicesCountsTabComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
