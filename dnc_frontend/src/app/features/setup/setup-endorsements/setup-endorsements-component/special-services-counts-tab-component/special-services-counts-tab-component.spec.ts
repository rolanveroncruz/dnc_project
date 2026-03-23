import { ComponentFixture, TestBed } from '@angular/core/testing';

import { SpecialServicesCountsTabComponent } from './special-services-counts-tab-component';

describe('SpecialServicesCountsTabComponent', () => {
  let component: SpecialServicesCountsTabComponent;
  let fixture: ComponentFixture<SpecialServicesCountsTabComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [SpecialServicesCountsTabComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(SpecialServicesCountsTabComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
