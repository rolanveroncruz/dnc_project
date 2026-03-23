import { ComponentFixture, TestBed } from '@angular/core/testing';

import { BasicServicesCountsTabComponent } from './basic-services-counts-tab-component';

describe('BasicServicesCountsTabComponent', () => {
  let component: BasicServicesCountsTabComponent;
  let fixture: ComponentFixture<BasicServicesCountsTabComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [BasicServicesCountsTabComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(BasicServicesCountsTabComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
